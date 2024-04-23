use std::{
    fs::{create_dir, remove_file, rename, File},
    path::PathBuf,
    sync::Arc,
};

use downloader::{progress::Reporter, Downloader};
use tar::Archive;
use wincompatlib::wine::ext::WineBootExt;
use xz::read::XzDecoder;

use super::component_downloader::ComponentDownloader;
use crate::{game_state::GameState, utils::github_requester::GithubRequester};

pub struct WineComponent {
    path: PathBuf,
}

impl GithubRequester for WineComponent {}

impl ComponentDownloader for WineComponent {
    async fn install<P: Reporter + 'static>(&self, progress: Option<Arc<P>>) -> anyhow::Result<()> {
        let file_output = Self::download(
            &self
                .path
                .parent()
                .expect("Failed to get the parent directory of Wine")
                .to_path_buf(),
            progress,
        )
        .await?;
        Self::uncompress(file_output.clone(), self.path.clone()).await?;

        Ok(())
    }

    async fn download<P: Reporter + 'static>(
        output_dir: &PathBuf,
        progress: Option<Arc<P>>,
    ) -> anyhow::Result<PathBuf> {
        let release = Self::get_latest_github_release("GloriousEggroll", "wine-ge-custom").await?;

        let asset = release[0]
            .assets
            .get(1)
            .expect("Asset not found in the github release");

        let mut downloader = Downloader::builder()
            .download_folder(output_dir)
            .parallel_requests(1)
            .build()?;

        let mut dl = downloader::Download::new(&asset.browser_download_url);
        if let Some(p) = progress {
            dl = dl.progress(p);
        }

        let _result = downloader.async_download(&[dl]).await?;
        let file_location = output_dir.join(asset.name.clone());

        Ok(file_location)
    }

    async fn uncompress(file: PathBuf, new_directory_name: PathBuf) -> anyhow::Result<()> {
        tokio::task::spawn_blocking(move || {
            let tar_xz = File::open(file.clone()).unwrap();
            let tar = XzDecoder::new(tar_xz);
            let mut archive = Archive::new(tar);
            archive
                .unpack(new_directory_name.parent().unwrap())
                .unwrap();
            remove_file(file.clone()).unwrap();
            rename(
                file.to_str()
                    .unwrap()
                    .replace("wine-", "")
                    .strip_suffix(".tar.xz")
                    .unwrap(),
                new_directory_name,
            )
            .unwrap();
        })
        .await
        .unwrap();

        Ok(())
    }
}

impl WineComponent {
    pub fn new(path: PathBuf) -> Self {
        WineComponent { path }
    }

    pub fn init_wine(&self) -> wincompatlib::prelude::Wine {
        let wine_path = self.path.join("bin/wine");
        let wine_prefix = self.path.parent().unwrap().join("data");
        if !wine_prefix.exists() {
            create_dir(wine_prefix.clone()).unwrap()
        }

        let wine = wincompatlib::prelude::Wine::from_binary(wine_path);
        wine.update_prefix(Some(wine_prefix)).unwrap();

        wine
    }
}
