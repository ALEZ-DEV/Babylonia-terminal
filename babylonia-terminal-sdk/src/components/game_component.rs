use downloader::progress;
use downloader::progress::Reporter;
use downloader::Downloader;
use log::debug;
use log::info;
use serde::Deserialize;
use serde::Serialize;
use std::collections::TryReserveError;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;
use std::vec;
use tokio::fs::create_dir_all;

use super::component_downloader::ComponentDownloader;
use crate::utils::github_requester::GithubRequester;
use crate::utils::kuro_prod_api;

pub struct GameComponent {
    game_dir: PathBuf,
}

impl GameComponent {
    pub fn new(game_dir: PathBuf) -> Self {
        Self { game_dir }
    }
}

impl GithubRequester for GameComponent {}

impl ComponentDownloader for GameComponent {
    async fn install<P: downloader::progress::Reporter + 'static>(
        &self,
        progress: Option<std::sync::Arc<P>>,
    ) -> anyhow::Result<()> {
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

        if let Some(ref p) = progress {
            let mut size: u64 = 0;

            resources
                .resource
                .iter()
                .for_each(|r| size += r.size as u64);

            p.setup(Some(size), "download has started");
        }

        for chunk_resource in resources.resource.chunks(5) {
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

                    if let Some(p) = progress.clone() {
                        dl = dl.progress(FileDownloadReporter::create(p));
                    }
                    dl
                })
                .collect::<Vec<downloader::Download>>();

            downloader.async_download(&to_download).await?;
        }

        Ok(output_dir.clone())
    }

    async fn uncompress(
        file: std::path::PathBuf,
        new_filename: std::path::PathBuf,
    ) -> anyhow::Result<()> {
        panic!("How did you run this function??!!")
    }
}

struct FileDownloadReporterPrivate {
    last_update: Instant,
    max_progress: Option<u64>,
    last_current_progress: u64,
}

struct FileDownloadReporter<P: Reporter> {
    private: Mutex<Option<FileDownloadReporterPrivate>>,
    progress: Mutex<Arc<P>>,
}

impl<P: Reporter> FileDownloadReporter<P> {
    pub fn create(progress: Arc<P>) -> Arc<Self> {
        Arc::new(Self {
            private: Mutex::new(None),
            progress: Mutex::new(progress),
        })
    }
}

impl<P: Reporter> Reporter for FileDownloadReporter<P> {
    fn setup(&self, max_progress: Option<u64>, message: &str) {
        let private = FileDownloadReporterPrivate {
            last_update: Instant::now(),
            max_progress,
            last_current_progress: 0,
        };

        let mut guard = self.private.lock().unwrap();
        *guard = Some(private);
    }

    fn progress(&self, current: u64) {
        if let Some(private) = self.private.lock().unwrap().as_mut() {
            let progress = self.progress.lock().unwrap();
            if private.last_update.elapsed().as_millis() >= 250 {
                progress.progress(current - private.last_current_progress); // to add to the progress
                private.last_current_progress = current;
            }
            private.last_update = Instant::now();
        }
    }

    fn set_message(&self, message: &str) {}

    fn done(&self) {}
}
