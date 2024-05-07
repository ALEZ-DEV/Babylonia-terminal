use std::{path::PathBuf, sync::Arc};

use downloader::progress::Reporter;
use log::{debug, info};
use tokio::fs::create_dir_all;
use wincompatlib::prelude::*;

use crate::{
    components::{
        component_downloader::ComponentDownloader, dxvk_component::DXVKComponent,
        game_component::GameComponent, proton_component::ProtonComponent,
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

    pub async fn install_font(proton: &Proton) -> anyhow::Result<()> {
        let wine_with_proton_prefix = proton // wine take the data/wine/pfx prefix, but we want the data/wine prefix
            .wine()
            .clone()
            .with_prefix(proton.wine().prefix.parent().unwrap());

        info!("0/10 font installed");

        wine_with_proton_prefix.install_font(Font::Arial)?;
        info!("1/10 font installed");

        wine_with_proton_prefix.install_font(Font::Andale)?;
        info!("2/10 font installed");

        wine_with_proton_prefix.install_font(Font::Courier)?;
        info!("3/10 font installed");

        wine_with_proton_prefix.install_font(Font::ComicSans)?;
        info!("4/10 font installed");

        wine_with_proton_prefix.install_font(Font::Georgia)?;
        info!("5/10 font installed");

        wine_with_proton_prefix.install_font(Font::Impact)?;
        info!("6/10 font installed");

        wine_with_proton_prefix.install_font(Font::Times)?;
        info!("7/10 font installed");

        wine_with_proton_prefix.install_font(Font::Trebuchet)?;
        info!("8/10 font installed");

        wine_with_proton_prefix.install_font(Font::Verdana)?;
        info!("9/10 font installed");

        wine_with_proton_prefix.install_font(Font::Webdings)?;
        info!("10/10 font installed");

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

    // this function just pass is_game_installed and is_game_patched to false,
    // so the launcher on the next iteration download the new file and delete the old one with the check process in the installation process
    pub async fn update_game() -> anyhow::Result<()> {
        let mut config = GameState::get_config().await;
        config.is_game_installed = false;
        config.is_game_patched = false;
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
