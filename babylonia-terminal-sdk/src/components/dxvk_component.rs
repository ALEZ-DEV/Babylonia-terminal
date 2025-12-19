use std::{
    fs::{remove_file, rename, File},
    path::PathBuf,
};

use downloader::Downloader;
use flate2::read::GzDecoder;
use tar::Archive;
use tokio::fs::remove_dir_all;
use wincompatlib::{
    dxvk::InstallParams,
    wine::{ext::WineWithExt, Wine},
};

use crate::utils::github_requester::GithubRequester;

use super::component_downloader::ComponentDownloader;

pub static DXVK_DEV: &str = "doitsujin";
pub static DXVK_REPO: &str = "dxvk";

pub struct DXVKComponent<'a> {
    wine: &'a Wine,
    path: PathBuf,
    github_release_index: usize,
}

impl<'a> DXVKComponent<'a> {
    pub fn from_wine<'b: 'a>(wine: &'b Wine, path: PathBuf) -> Self {
        DXVKComponent {
            wine,
            path: path.join("dxvk"),
            github_release_index: 0,
        }
    }
}

impl<'a> GithubRequester for DXVKComponent<'a> {
    fn set_github_release_index(&mut self, new_release_index: usize) {
        self.github_release_index = new_release_index;
    }
}

impl<'a> ComponentDownloader for DXVKComponent<'a> {
    async fn install<P: downloader::progress::Reporter + 'static>(
        &self,
        progress: Option<std::sync::Arc<P>>,
    ) -> anyhow::Result<()> {
        let dir = self
            .path
            .parent()
            .expect("Failed to get parent folder for DXVK")
            .to_path_buf();
        let file_output = self.download(&dir, progress).await?;

        Self::uncompress(file_output.clone(), self.path.clone()).await?;

        self.wine
            .install_dxvk(self.path.clone(), InstallParams::default())
            .expect("Failed to installed DXVK");

        //clean the directory
        remove_dir_all(self.path.clone())
            .await
            .expect("Failed to delete all the dxvk files");

        Ok(())
    }

    async fn download<P: downloader::progress::Reporter + 'static>(
        &self,
        output_dir: &std::path::PathBuf,
        progress: Option<std::sync::Arc<P>>,
    ) -> anyhow::Result<std::path::PathBuf> {
        let releases =
            Self::get_github_release_version(DXVK_DEV, DXVK_REPO, self.github_release_index)
                .await?;

        let asset = releases
            .assets
            .first()
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

    async fn uncompress(
        file: std::path::PathBuf,
        new_directory_name: std::path::PathBuf,
    ) -> anyhow::Result<()> {
        tokio::task::spawn_blocking(move || -> anyhow::Result<()> {
            let tar_gz = File::open(file.clone())?;
            let tar = GzDecoder::new(tar_gz);
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
