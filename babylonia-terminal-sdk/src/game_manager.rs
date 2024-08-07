use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
    process::{Child, Command, Stdio},
    sync::Arc,
};

use downloader::progress::Reporter;
use log::{debug, info};
use tokio::fs::create_dir_all;
use wincompatlib::prelude::*;

use crate::{
    components::{
        component_downloader::ComponentDownloader, dxvk_component::DXVKComponent,
        game_component::GameComponent, proton_component::ProtonComponent,
    },
    game_config::GameConfig,
    game_patcher,
    utils::{get_game_name, get_game_name_with_executable, github_requester::GithubRequester},
};

pub struct GameManager;

impl GameManager {
    pub async fn install_wine<P>(
        config_dir: PathBuf,
        release_index: usize,
        progress: Option<Arc<P>>,
    ) -> anyhow::Result<ProtonComponent>
    where
        P: Reporter + 'static,
    {
        let mut wine_component = ProtonComponent::new(config_dir);
        wine_component.set_github_release_index(release_index);

        wine_component.install(progress).await?;

        let mut config = GameConfig::get_config().await;
        config.is_wine_installed = true;
        GameConfig::save_config(config).await?;

        Ok(wine_component)
    }

    pub async fn install_dxvk<P>(
        proton: &Proton,
        config_dir: PathBuf,
        release_index: usize,
        progress: Option<Arc<P>>,
    ) -> anyhow::Result<()>
    where
        P: Reporter + 'static,
    {
        let mut dxvk_component = DXVKComponent::from_wine(proton.wine(), config_dir);
        dxvk_component.set_github_release_index(release_index);

        dxvk_component.install(progress).await?;

        let mut config = GameConfig::get_config().await;
        config.is_dxvk_installed = true;
        GameConfig::save_config(config).await?;

        Ok(())
    }

    pub async fn install_font<P>(proton: &Proton, progress: Option<Arc<P>>) -> anyhow::Result<()>
    where
        P: Reporter + 'static,
    {
        let wine_with_proton_prefix = proton // wine take the data/wine/pfx prefix, but we want the data/wine prefix
            .wine()
            .clone()
            .with_prefix(proton.wine().prefix.parent().unwrap());
        if let Some(p) = &progress {
            p.setup(Some(10), "");
        }

        notify_fonts_progress(0, &progress);

        wine_with_proton_prefix.install_font(Font::Arial)?;
        notify_fonts_progress(1, &progress);

        wine_with_proton_prefix.install_font(Font::Andale)?;
        notify_fonts_progress(2, &progress);

        wine_with_proton_prefix.install_font(Font::Courier)?;
        notify_fonts_progress(3, &progress);

        wine_with_proton_prefix.install_font(Font::ComicSans)?;
        notify_fonts_progress(4, &progress);

        wine_with_proton_prefix.install_font(Font::Georgia)?;
        notify_fonts_progress(5, &progress);

        wine_with_proton_prefix.install_font(Font::Impact)?;
        notify_fonts_progress(6, &progress);

        wine_with_proton_prefix.install_font(Font::Times)?;
        notify_fonts_progress(7, &progress);

        wine_with_proton_prefix.install_font(Font::Trebuchet)?;
        notify_fonts_progress(8, &progress);

        wine_with_proton_prefix.install_font(Font::Verdana)?;
        notify_fonts_progress(9, &progress);

        wine_with_proton_prefix.install_font(Font::Webdings)?;
        notify_fonts_progress(10, &progress);

        let mut config = GameConfig::get_config().await;
        config.is_font_installed = true;
        GameConfig::save_config(config).await?;

        Ok(())
    }

    pub async fn install_dependencies(proton: &Proton) -> anyhow::Result<()> {
        let wine_with_proton_prefix = proton // wine take the data/wine/pfx prefix, but we want the data/wine prefix
            .wine()
            .clone()
            .with_prefix(proton.wine().prefix.parent().unwrap());

        let winetricks = Winetricks::from_wine("/bin/winetricks", wine_with_proton_prefix);
        //winetricks.install("corefonts")?;
        let mut child = winetricks.install("vcrun2022")?;

        child
            .wait()
            .expect("Something failed when waiting for the installation");

        let mut config = GameConfig::get_config().await;
        config.is_dependecies_installed = true;
        GameConfig::save_config(config).await?;

        Ok(())
    }

    pub async fn install_game<P>(game_dir: PathBuf, progress: Arc<P>) -> anyhow::Result<()>
    where
        P: Reporter + 'static,
    {
        let _ = create_dir_all(game_dir.clone()).await;

        let game_component = GameComponent::new(game_dir);
        game_component.install(Some(progress)).await?;

        let mut config = GameConfig::get_config().await;
        config.is_game_installed = true;
        GameConfig::save_config(config).await?;

        Ok(())
    }

    // this function just pass is_game_installed and is_game_patched to false,
    // so the launcher on the next iteration download the new file and delete the old one with the check process in the installation process
    pub async fn update_game() -> anyhow::Result<()> {
        let mut config = GameConfig::get_config().await;
        config.is_game_installed = false;
        config.is_game_patched = false;
        GameConfig::save_config(config).await?;

        Ok(())
    }

    pub async fn patch_game(game_dir: PathBuf) -> anyhow::Result<()> {
        game_patcher::patch_game(game_dir).await?;

        Ok(())
    }

    pub async fn start_game(proton: &Proton, game_dir: PathBuf, options: Option<String>) {
        let proton_version = proton.wine().version().unwrap();
        let binary_path = game_dir
            .join(get_game_name())
            .join(get_game_name_with_executable());

        debug!("Wine version : {:?}", proton_version);

        let mut child = if let Some(custom_command) = options {
            Self::run(proton, binary_path, custom_command)
                .await
                .unwrap()
        } else {
            if let Some(custom_command) = GameConfig::get_launch_options().await.unwrap() {
                Self::run(proton, binary_path, custom_command)
                    .await
                    .unwrap()
            } else {
                debug!("Starting game without --options");
                proton.run(binary_path).unwrap()
            }
        };

        let stdout = child.stdout.take().unwrap();
        let mut bufread = BufReader::new(stdout);
        let mut buf = String::new();

        while let Ok(n) = bufread.read_line(&mut buf) {
            if n > 0 {
                info!("[Wine {:?}] : {}", proton_version, buf.trim());
                buf.clear();
            } else {
                break;
            }
        }
    }

    async fn run(
        proton: &Proton,
        binary_path: PathBuf,
        custom_command: String,
    ) -> Result<Child, std::io::Error> {
        debug!("Starting game with --options -> {}", custom_command);
        let tokens: Vec<&str> = custom_command.split_whitespace().collect();

        // position of the %command%
        let index = tokens
            .iter()
            .position(|&s| s == "%command%")
            .expect("You forget to put %command% in your custom launch command");

        Command::new(tokens.get(0).unwrap())
            .args(&tokens[0..(index - 1)])
            .arg(proton.python.as_os_str())
            .arg(
                GameConfig::get_config_directory()
                    .await
                    .join("proton")
                    .join("proton"),
            )
            .arg("run")
            .arg(binary_path)
            .args(&tokens[(index + 1)..tokens.len()])
            .envs(proton.get_envs())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
    }
}

fn notify_fonts_progress<P>(nbr: u64, progress: &Option<Arc<P>>)
where
    P: Reporter + 'static,
{
    info!("{}", format!("{}/10 font installed", nbr));
    if let Some(p) = progress {
        p.progress(nbr);
    }
}
