use std::{
    fs::{create_dir, remove_file, rename, File},
    path::PathBuf,
    sync::Arc,
    time::Duration,
};

use downloader::progress::Reporter;
use flate2::read::GzDecoder;
use log::info;
use tar::Archive;
use tokio::{join, time::sleep};
use wincompatlib::{prelude::*, wine::bundle::proton};
use xz::read::XzDecoder;

use crate::{
    components::{component_downloader::ComponentDownloader, wine_component::WineComponent},
    components_downloader::ComponentsDownloader,
    game_state::GameState,
};

pub struct GameManager;

impl GameManager {
    pub async fn install_wine<P>(
        config_dir: PathBuf,
        progress: Option<Arc<P>>,
    ) -> anyhow::Result<WineComponent>
    where
        P: Reporter + 'static,
    {
        let wine_component = WineComponent::new(config_dir.join("wine"));

        wine_component.install(progress).await?;

        let mut config = GameState::get_config().await?;
        config.is_wine_installed = true;
        config.wine_path = Some(
            GameState::get_config_directory()
                .join("wine")
                .to_str()
                .unwrap()
                .to_string(),
        );
        GameState::save_config(config).await?;

        Ok(wine_component)
    }

    pub async fn install_dxvk<P>(
        wine: &Wine,
        config_dir: PathBuf,
        progress: Option<Arc<P>>,
    ) -> anyhow::Result<()>
    where
        P: Reporter + 'static,
    {
        let file_output = ComponentsDownloader::download_latest_dxvk(&config_dir, progress)
            .await
            .expect("failed to download dxvk");

        let file_to_uncompress = file_output.clone();
        tokio::task::spawn_blocking(move || {
            let tar_gz = File::open(file_to_uncompress.clone()).unwrap();
            let tar = GzDecoder::new(tar_gz);
            let mut archive = Archive::new(tar);
            archive.unpack(config_dir).unwrap();
            remove_file(file_to_uncompress.clone()).unwrap();
        })
        .await
        .unwrap();

        wine.install_dxvk(
            file_output
                .to_str()
                .unwrap()
                .strip_suffix(".tar.gz")
                .unwrap(),
            InstallParams::default(),
        )
        .expect("failed to installed DXVK");

        let mut config = GameState::get_config().await?;
        config.is_dxvk_installed = true;
        GameState::save_config(config).await?;

        Ok(())
    }

    pub async fn install_font<P>(wine: &Wine, progress: Arc<P>) -> anyhow::Result<()>
    where
        P: Reporter + 'static,
    {
        progress.setup(Some(10), "");
        progress.progress(0);

        wine.install_font(Font::Arial)?;
        progress.progress(1);

        wine.install_font(Font::Andale)?;
        progress.progress(2);

        wine.install_font(Font::Courier)?;
        progress.progress(3);

        wine.install_font(Font::ComicSans)?;
        progress.progress(4);

        wine.install_font(Font::Georgia)?;
        progress.progress(5);

        wine.install_font(Font::Impact)?;
        progress.progress(6);

        wine.install_font(Font::Times)?;
        progress.progress(7);

        wine.install_font(Font::Trebuchet)?;
        progress.progress(8);

        wine.install_font(Font::Verdana)?;
        progress.progress(9);

        wine.install_font(Font::Webdings)?;
        progress.progress(10);

        let mut config = GameState::get_config().await?;
        config.is_font_installed = true;
        GameState::save_config(config).await?;

        Ok(())
    }

    pub async fn install_dependecies(wine: &Wine) -> anyhow::Result<()> {
        let winetricks = Winetricks::from_wine("winetricks", wine);
        winetricks.install("corefonts")?;
        winetricks.install("vcrun2022")?;

        let mut config = GameState::get_config().await?;
        config.is_dependecies_installed = true;
        GameState::save_config(config).await?;

        Ok(())
    }

    pub fn init_wine() -> Wine {
        let wine_path = GameState::get_config_directory().join("wine/bin/wine");
        let wine_prefix = GameState::get_config_directory().join("data");
        if !wine_prefix.exists() {
            create_dir(wine_prefix.clone()).unwrap()
        }

        let wine = Wine::from_binary(wine_path);
        wine.update_prefix(Some(wine_prefix)).unwrap();

        wine
    }

    pub fn download_game() {}

    pub async fn start_game(wine: &Wine) {
        wine.run("/home/alez/.steam/steam/steamapps/compatdata/3841903579/pfx/drive_c/Punishing Gray Raven/launcher.exe").unwrap();

        loop {
            sleep(Duration::from_millis(10000)).await;
        }
    }
}
