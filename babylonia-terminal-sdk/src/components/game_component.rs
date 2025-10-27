use std::fs as std_fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::vec;

use downloader::progress::Reporter;
use downloader::Downloader;
use futures::future::join_all;
use log::debug;
use log::info;
use tokio::fs::{create_dir_all, remove_file};
use tokio::sync::Semaphore;

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
        let to_download: Arc<Mutex<Vec<Resource>>> = Arc::new(Mutex::new(vec![]));

        let threads_number = num_cpus::get();
        debug!("Starting Md5 files check with {} threads", threads_number);
        let mut handles = vec![];
        let semaphore = Arc::new(Semaphore::new(threads_number));

        let chunks = resources.resource.chunks(threads_number);

        for chunked_resources in chunks {
            let to_download_ref = to_download.clone();
            let cloned_resources = chunked_resources.to_owned();
            let game_dir = game_dir.clone();
            let semaphore = semaphore.clone();

            let handle = tokio::task::spawn(async move {
                let _permit = semaphore.acquire().await;

                for resource_to_check in cloned_resources {
                    let file_path = game_dir.join(resource_to_check.dest.clone());

                    if file_path.try_exists().unwrap() {
                        let blocking_file_path = file_path.clone();
                        let file = tokio::task::spawn_blocking(move || {
                            std_fs::File::open(blocking_file_path)
                        })
                        .await
                        .unwrap()
                        .unwrap(); // only the std::File is supported by chksum_md5, that's why I block
                        let digest = chksum_md5::chksum(file)
                            .expect("Failed to check the checksum of the file");

                        if digest.to_hex_lowercase() != resource_to_check.md5 {
                            to_download_ref
                                .lock()
                                .as_mut()
                                .unwrap()
                                .push(resource_to_check.clone());
                            remove_file(file_path).await.unwrap();
                        }
                    } else {
                        to_download_ref
                            .lock()
                            .as_mut()
                            .unwrap()
                            .push(resource_to_check.clone());
                    }
                }
            });
            handles.push(handle);
        }

        join_all(handles).await;

        Ok(Arc::try_unwrap(to_download)
            .expect("There is multiple references of this Vector")
            .into_inner()?)
    }
}

impl GithubRequester for GameComponent {
    fn set_github_release_index(&mut self, _: usize) {}
}

impl ComponentDownloader for GameComponent {
    async fn install<P: downloader::progress::Reporter + 'static>(
        &self,
        progress: Option<std::sync::Arc<P>>,
    ) -> anyhow::Result<()> {
        let _ = create_dir_all(&self.game_dir).await;
        self.download(&self.game_dir, progress).await?;

        Ok(())
    }

    async fn download<P: downloader::progress::Reporter + 'static>(
        &self,
        output_dir: &std::path::PathBuf,
        progress: Option<std::sync::Arc<P>>,
    ) -> anyhow::Result<std::path::PathBuf> {
        let game_info = kuro_prod_api::GameInfo::get_info().await?;
        let resources = game_info.fetch_resources().await?;

        let threads_number = num_cpus::get();

        let mut downloader = Downloader::builder()
            .download_folder(output_dir)
            .parallel_requests(threads_number as u16)
            .build()?;

        let checked_resources =
            GameComponent::check_and_get_resources_to_download(output_dir, &resources).await?;

        let mut global_reporter = None;

        if let Some(p) = progress {
            let max_size: u64 = checked_resources.iter().map(|r| r.size as u64).sum();

            global_reporter = Some(Arc::new(Mutex::new(GlobalReporter::new(p, max_size))));
            global_reporter.clone().unwrap().lock().unwrap().setup();
        }

        for chunk_resource in checked_resources.chunks(threads_number) {
            let mut output_path: Vec<PathBuf> = vec![];

            for path in chunk_resource
                .iter()
                .map(|r| {
                    output_dir
                        .join(r.dest.clone())
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

                    if let Some(gr) = global_reporter.clone() {
                        dl = dl.progress(Arc::new(FileReporter::new(gr)));
                    }

                    dl
                })
                .collect::<Vec<downloader::Download>>();

            downloader.async_download(&to_download).await?;
        }

        if let Some(gr) = global_reporter {
            gr.lock().unwrap().done();
        }

        Ok(output_dir.clone())
    }

    async fn uncompress(
        _file: std::path::PathBuf,
        _new_filename: std::path::PathBuf,
    ) -> anyhow::Result<()> {
        anyhow::bail!("How did you run this function??!!")
    }
}

struct GlobalReporter<P: downloader::progress::Reporter + 'static> {
    progress: Arc<P>,
    pub to_download_max_size: u64,
    current_progress: Mutex<u64>,
}

impl<P: downloader::progress::Reporter + 'static> GlobalReporter<P> {
    pub fn new(progress: Arc<P>, max_size: u64) -> Self {
        Self {
            progress,
            to_download_max_size: max_size,
            current_progress: Mutex::new(0),
        }
    }

    pub fn setup(&self) {
        self.progress.setup(Some(self.to_download_max_size), "");
    }

    pub fn update(&self, new_value: u64, old_value: u64) {
        let mut current_progress = self.current_progress.lock().unwrap();
        let current = current_progress.clone() + new_value - old_value;
        *current_progress = current;
        self.progress.progress(current);
    }

    pub fn done(&self) {
        self.progress.done();
    }
}

struct FileReporter<P: downloader::progress::Reporter + 'static> {
    global_reporter: Arc<Mutex<GlobalReporter<P>>>,
    old_current: Mutex<u64>,
}

impl<P: downloader::progress::Reporter + 'static> FileReporter<P> {
    pub fn new(global_reporter: Arc<Mutex<GlobalReporter<P>>>) -> Self {
        Self {
            global_reporter,
            old_current: Mutex::new(0),
        }
    }
}

impl<P: downloader::progress::Reporter + 'static> Reporter for FileReporter<P> {
    fn setup(&self, _: Option<u64>, _: &str) {}

    fn progress(&self, current: u64) {
        let p = self.global_reporter.lock().unwrap();
        let mut old_current_guard = self.old_current.lock().unwrap();
        let old_current = old_current_guard.clone();

        *old_current_guard = current;

        p.update(current, old_current);
    }

    fn set_message(&self, _: &str) {}

    fn done(&self) {}
}
