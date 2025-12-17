use std::{
    fs::{remove_file, rename, File},
    path::PathBuf,
    str::FromStr,
    sync::Arc,
};

use downloader::{progress::Reporter, Downloader};
use log::debug;
use tar::Archive;
use wincompatlib::wine::ext::WineBootExt;
use xz2::read::XzDecoder;

use super::component_downloader::ComponentDownloader;
use crate::utils::github_requester::GithubRequester;

pub static WINE_DEV: &str = "Kron4ek";
pub static WINE_REPO: &str = "Wine-Builds";

#[derive(Debug, PartialEq, Eq)]
pub struct WineComponent {
    path: PathBuf,
    github_release_index: usize,
}

impl GithubRequester for WineComponent {
    fn set_github_release_index(&mut self, new_release_index: usize) {
        self.github_release_index = new_release_index;
    }
}

impl ComponentDownloader for WineComponent {
    async fn install<P: Reporter + 'static>(&self, progress: Option<Arc<P>>) -> anyhow::Result<()> {
        let file_output = self
            .download(
                &self
                    .path
                    .parent()
                    .expect("Failed to get the parent directory of wine")
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
            Self::get_github_release_version(WINE_DEV, WINE_REPO, self.github_release_index)
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
            let tar = XzDecoder::new(tar_xz);
            let mut archive = Archive::new(tar);
            archive.unpack(new_directory_name.parent().unwrap())?;
            remove_file(file.clone())?;
            rename(
                file.to_str().unwrap().strip_suffix(".tar.xz").unwrap(),
                new_directory_name,
            )?;

            Ok::<(), anyhow::Error>(())
        })
        .await??;

        Ok(())
    }
}

impl WineComponent {
    pub fn new(path: PathBuf) -> Self {
        WineComponent {
            path: path.join("wine"),
            github_release_index: 0,
        }
    }

    pub fn init_wine(&self) -> Result<wincompatlib::prelude::Wine, String> {
        let prefix = self.path.parent().unwrap().join("data");
        let wine_bin_location = self.path.join("bin/wine");
        debug!("Initializing prefix with -> {:?}", prefix);
        debug!("Wine binary path : {:?}", wine_bin_location);

        let mut wine = wincompatlib::prelude::Wine::from_binary(wine_bin_location);

        wine.init_prefix(Some(prefix)).unwrap();

        Ok(wine)
    }

    //fn get_steam_location() -> Result<PathBuf, String> {
    //    let specified_steam_location = std::env::var("BT_STEAM_CLIENT_PATH");
    //    if let Ok(location) = specified_steam_location {
    //        return Ok(PathBuf::from(location));
    //    }

    //    let location_to_check = [
    //        dirs::home_dir().unwrap().join(".steam/steam"),
    //        dirs::home_dir()
    //            .unwrap()
    //            .join(".var/app/com.valvesoftware.Steam/steam"), // for the flatpak version of steam
    //    ];

    //    for location in location_to_check {
    //        if location.exists() {
    //            return Ok(location);
    //        }
    //    }

    //    debug!("Can't find steam installation");
    //    Err(String::from_str("We can't find your steam installation, please install steam in '~/.steam/steam' or specify your steam installation").unwrap())
    //}
}
