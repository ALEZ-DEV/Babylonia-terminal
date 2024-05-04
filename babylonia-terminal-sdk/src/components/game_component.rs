use downloader::progress;
use downloader::progress::Reporter;
use downloader::Downloader;
use log::debug;
use log::info;
use serde::Deserialize;
use serde::Serialize;
use std::collections::TryReserveError;
use std::fs as std_fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::time::Instant;
use std::vec;
use tokio::fs::File;
use tokio::fs::{create_dir_all, remove_file};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tokio::time::timeout;

use super::component_downloader::ComponentDownloader;
use crate::utils::get_game_name;
use crate::utils::github_requester::GithubRequester;
use crate::utils::kuro_prod_api;
use crate::utils::kuro_prod_api::Resource;
use crate::utils::kuro_prod_api::Resources;

pub struct GameComponent {
    game_dir: PathBuf,
}

impl GameComponent {
    pub fn new(game_dir: PathBuf) -> Self {
        Self {
            game_dir: game_dir.join(get_game_name()),
        }
    }

    async fn check_and_get_resources_to_download(
        game_dir: &std::path::PathBuf,
        resources: &Resources,
    ) -> anyhow::Result<Vec<Resource>> {
        info!("checking all files, this can take a while...");
        let mut to_download: Vec<Resource> = vec![];

        for r in &resources.resource {
            let file_path = game_dir.join(r.dest.clone().strip_prefix("/").unwrap());

            if file_path.try_exists()? {
                let blocking_file_path = file_path.clone();
                let file =
                    tokio::task::spawn_blocking(move || std_fs::File::open(blocking_file_path))
                        .await??; // only the std::File is supported by chksum_md5, that's why I block
                let digest = chksum_md5::chksum(file)?;

                if digest.to_hex_lowercase() != r.md5 {
                    to_download.push(r.clone());
                    remove_file(file_path).await?;
                }
            } else {
                to_download.push(r.clone());
            }
        }

        Ok(to_download)
    }

    fn update_progress<P: downloader::progress::Reporter + 'static>(
        progress: Arc<P>,
        max_size: u64,
        game_dir: PathBuf,
        mut rx: Receiver<&'static str>,
    ) {
        tokio::spawn(async move {
            progress.setup(Some(max_size.to_owned()), "");

            loop {
                let current_size = fs_extra::dir::get_size(game_dir.clone());
                if let Ok(size) = current_size {
                    progress.progress(size);
                }

                if let Ok(v) = timeout(Duration::from_millis(250), rx.recv()).await {
                    match v {
                        Some(str) => {
                            if str == "done" {
                                break;
                            }
                        }
                        _ => {}
                    }
                }
            }
        });
    }
}

impl GithubRequester for GameComponent {}

impl ComponentDownloader for GameComponent {
    async fn install<P: downloader::progress::Reporter + 'static>(
        &self,
        progress: Option<std::sync::Arc<P>>,
    ) -> anyhow::Result<()> {
        let _ = create_dir_all(&self.game_dir).await;
        Self::download(&self.game_dir, progress).await?;

        Ok(())
    }

    async fn download<P: downloader::progress::Reporter + 'static>(
        output_dir: &std::path::PathBuf,
        progress: Option<std::sync::Arc<P>>,
    ) -> anyhow::Result<std::path::PathBuf> {
        let game_info = kuro_prod_api::fetch_game_info().await?;
        let resources = game_info.fetch_resources().await?;
        let mut downloader = Downloader::builder()
            .download_folder(output_dir)
            .parallel_requests(5)
            .build()?;

        let checked_resources =
            GameComponent::check_and_get_resources_to_download(output_dir, &resources).await?;
        let (tx, rx): (Sender<&str>, Receiver<&str>) = mpsc::channel(100);

        if let Some(p) = progress {
            GameComponent::update_progress(
                p,
                resources.get_max_size_resources(),
                output_dir.clone(),
                rx,
            );
        }

        for chunk_resource in checked_resources.chunks(5) {
            let mut output_path: Vec<PathBuf> = vec![];

            for path in chunk_resource
                .iter()
                .map(|r| {
                    output_dir
                        .join(r.dest.strip_prefix("/").unwrap())
                        .parent()
                        .unwrap()
                        .to_path_buf()
                })
                .collect::<Vec<_>>()
            {
                let _ = create_dir_all(&path).await; // unecessary to check
                output_path.push(path)
            }

            let to_download = chunk_resource
                .iter()
                .enumerate()
                .map(|(i, r)| {
                    let url = r.build_download_url(
                        &game_info.get_first_cdn(),
                        &game_info.get_resource_base_path(),
                    );
                    debug!("starting download for {}", url);
                    let mut dl = downloader::Download::new_with_output(
                        &url,
                        output_path
                            .get(i)
                            .expect("Failed to get the write path of the concerned file")
                            .to_owned(),
                    );
                    dl.check_file_name = false;

                    dl
                })
                .collect::<Vec<downloader::Download>>();

            downloader.async_download(&to_download).await?;
        }

        let _ = tx.send("done").await;

        Ok(output_dir.clone())
    }

    async fn uncompress(
        file: std::path::PathBuf,
        new_filename: std::path::PathBuf,
    ) -> anyhow::Result<()> {
        panic!("How did you run this function??!!")
    }
}
