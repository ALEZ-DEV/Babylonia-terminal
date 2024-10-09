use std::{
    fs::{remove_file, rename, File},
    path::PathBuf,
    str::FromStr,
    sync::Arc,
};

use downloader::{progress::Reporter, Downloader};
use flate2::read::GzDecoder;
use log::debug;
use tar::Archive;
use wincompatlib::wine::ext::WineBootExt;

use super::component_downloader::ComponentDownloader;
use crate::utils::github_requester::GithubRequester;

pub static PROTON_DEV: &str = "GloriousEggroll";
pub static PROTON_REPO: &str = "proton-ge-custom";

#[derive(Debug, PartialEq, Eq)]
pub struct ProtonComponent {
    path: PathBuf,
    github_release_index: usize,
}

impl GithubRequester for ProtonComponent {
    fn set_github_release_index(&mut self, new_release_index: usize) {
        self.github_release_index = new_release_index;
    }
}

impl ComponentDownloader for ProtonComponent {
    async fn install<P: Reporter + 'static>(&self, progress: Option<Arc<P>>) -> anyhow::Result<()> {
        let file_output = self
            .download(
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
        &self,
        output_dir: &PathBuf,
        progress: Option<Arc<P>>,
    ) -> anyhow::Result<PathBuf> {
        let release =
            Self::get_github_release_version(PROTON_DEV, PROTON_REPO, self.github_release_index)
                .await?;

        let asset = release
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
            archive.unpack(new_directory_name.parent().unwrap())?;
            remove_file(file.clone())?;
            rename(
                file.to_str().unwrap().strip_suffix(".tar.gz").unwrap(),
                new_directory_name,
            )?;

            Ok::<(), anyhow::Error>(())
        })
        .await??;

        Ok(())
    }
}

impl ProtonComponent {
    pub fn new(path: PathBuf) -> Self {
        ProtonComponent {
            path: path.join("proton"),
            github_release_index: 0,
        }
    }

    pub fn init_proton(&self) -> Result<wincompatlib::prelude::Proton, String> {
        let prefix = self.path.parent().unwrap().join("data");

        let mut proton =
            wincompatlib::prelude::Proton::new(self.path.clone(), Some(prefix.clone()));
        let steam_location = Self::get_steam_location()?;
        proton.steam_client_path = Some(steam_location);
        proton.init_prefix(Some(prefix)).unwrap();

        Ok(proton)
    }

    fn get_steam_location() -> Result<PathBuf, String> {
        let location_to_check = [
            dirs::home_dir().unwrap().join(".steam/steam"),
            dirs::home_dir()
                .unwrap()
                .join("/.var/app/com.valvesoftware.Steam/steam"), // for the flatpak version of steam
        ];

        for location in location_to_check {
            if location.exists() {
                return Ok(location);
            }
        }

        debug!("Can't find steam installation");
        Err(String::from_str("We can't find your steam installation, please install steam in '~/.steam/steam' or specify your steam installation").unwrap())
    }
}
