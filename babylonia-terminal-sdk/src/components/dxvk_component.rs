use std::{
    fs::{remove_dir, remove_file, rename, File},
    path::PathBuf,
};

use downloader::{Download, Downloader};
use flate2::read::GzDecoder;
use tar::Archive;
use tokio::fs::remove_dir_all;
use wincompatlib::{
    dxvk::InstallParams,
    wine::{ext::WineWithExt, Wine},
};

use crate::utils::github_requester::GithubRequester;

use super::component_downloader::ComponentDownloader;

pub struct DXVKComponent<'a> {
    wine: &'a Wine,
    path: PathBuf,
}

impl<'a> DXVKComponent<'a> {
    pub fn from_wine<'b: 'a>(wine: &'b Wine, path: PathBuf) -> Self {
        DXVKComponent { wine, path }
    }
}

impl<'a> GithubRequester for DXVKComponent<'a> {}

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
        let file_output = Self::download(&dir, progress).await?;

        Self::uncompress(file_output.clone(), self.path.clone()).await?;

        let wine_with_proton_prefix = self // wine take the data/wine/pfx prefix, but we want the data/wine prefix
            .wine
            .clone()
            .with_prefix(self.wine.prefix.parent().unwrap());

        wine_with_proton_prefix
            .install_dxvk(self.path.clone(), InstallParams::default())
            .expect("Failed to installed DXVK");

        //clean the directory
        remove_dir_all(self.path.clone())
            .await
            .expect("Failed to delete all the dxvk files");

        Ok(())
    }

    async fn download<P: downloader::progress::Reporter + 'static>(
        output_dir: &std::path::PathBuf,
        progress: Option<std::sync::Arc<P>>,
    ) -> anyhow::Result<std::path::PathBuf> {
        let releases = Self::get_latest_github_release("doitsujin", "dxvk").await?;

        let asset = releases[0]
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
        tokio::task::spawn_blocking(move || {
            let tar_gz = File::open(file.clone()).unwrap();
            let tar = GzDecoder::new(tar_gz);
            let mut archive = Archive::new(tar);
            archive
                .unpack(new_directory_name.parent().unwrap())
                .unwrap();
            remove_file(file.clone()).unwrap();
            rename(
                file.to_str().unwrap().strip_suffix(".tar.gz").unwrap(),
                new_directory_name,
            )
            .unwrap()
        })
        .await?;

        Ok(())
    }
}
