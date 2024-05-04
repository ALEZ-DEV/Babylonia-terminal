use std::{
    fs::{create_dir, remove_file, rename, File},
    io::{BufRead, BufReader},
    path::PathBuf,
    str::FromStr,
    sync::Arc,
    time::Duration,
};

use downloader::progress::Reporter;
use flate2::read::GzDecoder;
use log::{debug, info};
use tar::Archive;
use tokio::fs::create_dir_all;
use tokio::time::sleep;
use wincompatlib::{prelude::*, wine::bundle::proton};
use xz::read::XzDecoder;

use crate::{
    components::{
        component_downloader::ComponentDownloader,
        dxvk_component::{self, DXVKComponent},
        game_component::GameComponent,
        proton_component::ProtonComponent,
    },
    game_patcher,
    game_state::GameState,
    utils::{get_game_name, get_game_name_with_executable},
};

pub struct GameManager;

impl GameManager {
    pub async fn install_wine<P>(
        config_dir: PathBuf,
        progress: Option<Arc<P>>,
    ) -> anyhow::Result<ProtonComponent>
    where
        P: Reporter + 'static,
    {
        let wine_component = ProtonComponent::new(config_dir);

        wine_component.install(progress).await?;

        let mut config = GameState::get_config().await;
        config.is_wine_installed = true;
        GameState::save_config(config).await?;

        Ok(wine_component)
    }

    pub async fn install_dxvk<P>(
        proton: &Proton,
        config_dir: PathBuf,
        progress: Option<Arc<P>>,
    ) -> anyhow::Result<()>
    where
        P: Reporter + 'static,
    {
        let dxvk_component = DXVKComponent::from_wine(proton.wine(), config_dir);
        dxvk_component.install(progress).await?;

        let mut config = GameState::get_config().await;
        config.is_dxvk_installed = true;
        GameState::save_config(config).await?;

        Ok(())
    }

    pub async fn install_font<P>(proton: &Proton, progress: Arc<P>) -> anyhow::Result<()>
    where
        P: Reporter + 'static,
    {
        let wine_with_proton_prefix = proton // wine take the data/wine/pfx prefix, but we want the data/wine prefix
            .wine()
            .clone()
            .with_prefix(proton.wine().prefix.parent().unwrap());

        progress.setup(Some(10), "");
        progress.progress(0);

        wine_with_proton_prefix.install_font(Font::Arial)?;
        progress.progress(1);

        wine_with_proton_prefix.install_font(Font::Andale)?;
        progress.progress(2);

        wine_with_proton_prefix.install_font(Font::Courier)?;
        progress.progress(3);

        wine_with_proton_prefix.install_font(Font::ComicSans)?;
        progress.progress(4);

        wine_with_proton_prefix.install_font(Font::Georgia)?;
        progress.progress(5);

        wine_with_proton_prefix.install_font(Font::Impact)?;
        progress.progress(6);

        wine_with_proton_prefix.install_font(Font::Times)?;
        progress.progress(7);

        wine_with_proton_prefix.install_font(Font::Trebuchet)?;
        progress.progress(8);

        wine_with_proton_prefix.install_font(Font::Verdana)?;
        progress.progress(9);

        wine_with_proton_prefix.install_font(Font::Webdings)?;
        progress.progress(10);

        let mut config = GameState::get_config().await;
        config.is_font_installed = true;
        GameState::save_config(config).await?;

        Ok(())
    }

    pub async fn install_dependecies(proton: &Proton) -> anyhow::Result<()> {
        let wine_with_proton_prefix = proton // wine take the data/wine/pfx prefix, but we want the data/wine prefix
            .wine()
            .clone()
            .with_prefix(proton.wine().prefix.parent().unwrap());

        let winetricks = Winetricks::from_wine("/bin/winetricks", wine_with_proton_prefix);
        //winetricks.install("corefonts")?;
        winetricks.install("vcrun2022")?;

        let mut config = GameState::get_config().await;
        config.is_dependecies_installed = true;
        GameState::save_config(config).await?;

        Ok(())
    }

    pub async fn install_game<P>(game_dir: PathBuf, progress: Arc<P>) -> anyhow::Result<()>
    where
        P: Reporter + 'static,
    {
        let _ = create_dir_all(game_dir.clone()).await;

        let game_component = GameComponent::new(game_dir);
        game_component.install(Some(progress)).await?;

        let mut config = GameState::get_config().await;
        config.is_game_installed = true;
        GameState::save_config(config).await?;

        Ok(())
    }

    pub async fn patch_game(game_dir: PathBuf) -> anyhow::Result<()> {
        game_patcher::patch_game(game_dir).await?;

        Ok(())
    }

    pub async fn start_game(proton: &Proton, game_dir: PathBuf) {
        debug!("Wine version : {:?}", proton.wine().version().unwrap());
        let mut child = proton
            .run(
                game_dir
                    .join(get_game_name())
                    .join(get_game_name_with_executable()),
            )
            .unwrap();
        child.wait().expect("The game failed to run");
    }
}
