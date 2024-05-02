use std::{
    env::home_dir,
    fs::{create_dir, remove_file, rename, File},
    path::PathBuf,
    str::FromStr,
    sync::Arc,
};

use downloader::{progress::Reporter, Downloader};
use flate2::read::GzDecoder;
use log::debug;
use tar::Archive;
use wincompatlib::wine::{bundle::proton::Proton, ext::WineBootExt};
use xz::read::XzDecoder;

use super::component_downloader::ComponentDownloader;
use crate::{game_state::GameState, utils::github_requester::GithubRequester};

#[derive(Debug, PartialEq, Eq)]
pub struct ProtonComponent {
    path: PathBuf,
}

impl GithubRequester for ProtonComponent {}

impl ComponentDownloader for ProtonComponent {
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
        let release =
            Self::get_latest_github_release("GloriousEggroll", "proton-ge-custom").await?;

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
            let tar = GzDecoder::new(tar_xz);
            let mut archive = Archive::new(tar);
            archive
                .unpack(new_directory_name.parent().unwrap())
                .unwrap();
            remove_file(file.clone()).unwrap();
            rename(
                file.to_str().unwrap().strip_suffix(".tar.gz").unwrap(),
                new_directory_name,
            )
            .unwrap();
        })
        .await
        .unwrap();

        Ok(())
    }
}

impl ProtonComponent {
    pub fn new(path: PathBuf) -> Self {
        ProtonComponent { path }
    }

    pub fn init_proton(&self) -> Result<wincompatlib::prelude::Proton, String> {
        let prefix = self.path.parent().unwrap().join("data");
        let steam_location = dirs::home_dir().unwrap().join(".steam/steam");
        if !steam_location.exists() {
            debug!("Can't find steam installation");
            return Err(String::from_str("We can't find your steam installation, please install steam in '~/.steam/steam' or specify your steam installation").unwrap());
        }
        let mut proton =
            wincompatlib::prelude::Proton::new(self.path.clone(), Some(prefix.clone()));
        proton.steam_client_path = Some(steam_location);
        proton.init_prefix(Some(prefix)).unwrap();

        Ok(proton)
    }
}
