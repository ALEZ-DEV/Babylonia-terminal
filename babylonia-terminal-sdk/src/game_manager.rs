use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
    process::{Child, Command, Stdio},
    sync::{Arc, Mutex},
};

use downloader::progress::Reporter;
use log::{debug, info};
use tokio::{
    fs::{create_dir_all, remove_file, File, OpenOptions},
    io::AsyncWriteExt,
};
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

        let max = 1;

        if let Some(p) = &progress {
            p.setup(Some(max), "");
        }

        notify_fonts_progress(0, max, &progress);

        wine_with_proton_prefix.install_font(Font::Arial)?;
        notify_fonts_progress(1, max, &progress);

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

    pub async fn start_game(
        proton: &Proton,
        game_dir: PathBuf,
        options: Option<String>,
        show_logs: bool,
    ) -> anyhow::Result<()> {
        let proton_version = proton.wine().version()?;
        let binary_path = game_dir
            .join(get_game_name())
            .join(get_game_name_with_executable());

        debug!("Wine version : {:?}", proton_version);

        let mut child = if let Some(custom_command) = options {
            Self::run(proton, binary_path, Some(custom_command)).await?
        } else {
            if let Some(custom_command) = GameConfig::get_launch_options().await.unwrap() {
                Self::run(proton, binary_path, Some(custom_command)).await?
            } else {
                Self::run(proton, binary_path, None).await?
            }
        }?;

        let log_stdout = Arc::new(Mutex::new(None));
        let log_stderr = Arc::new(Mutex::new(None));

        if show_logs {
            let stderr = child.stderr.take().unwrap();
            let stdout = child.stdout.take().unwrap();

            let log_stdout_ref = log_stdout.clone();
            let log_stderr_ref = log_stderr.clone();

            tokio::task::spawn(async move {
                let bufread = BufReader::new(stdout);
                let mut stdout_save = String::new();
                let _: Vec<_> = bufread
                    .lines()
                    .inspect(|s| {
                        if let Ok(str) = s {
                            info!("[Proton] > {}", str);
                            stdout_save.push_str(str);
                        }
                    })
                    .collect();
                *log_stdout_ref.lock().unwrap() = Some(stdout_save);
            });
            tokio::task::spawn(async move {
                let bufread = BufReader::new(stderr);
                let mut stderr_save = String::new();
                let _: Vec<_> = bufread
                    .lines()
                    .inspect(|s| {
                        if let Ok(str) = s {
                            info!("[Proton] > {}", str);
                            stderr_save.push_str(str);
                        }
                    })
                    .collect();
                *log_stderr_ref.lock().unwrap() = Some(stderr_save);
            });
        }

        let output = child
            .wait_with_output()
            .expect("Failed to wait for the process");

        let log_file_path = GameConfig::get_config_directory().await.join("game.log");
        if log_file_path.exists() {
            remove_file(log_file_path.clone())
                .await
                .expect("Failed to remove old log file");
        }
        File::create(log_file_path.clone())
            .await
            .expect("Failed to create build file");

        let mut log_file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(log_file_path)
            .await
            .expect("Failed to open the log file");

        log_file
            .write_all("--- stdout ---\n".as_bytes())
            .await
            .expect("Failed to write the output to the log file");

        let log_stdout_value = log_stdout.lock().unwrap();
        let v;
        let to_write = if log_stdout_value.is_some() {
            v = log_stdout_value.clone().unwrap();
            v.as_bytes()
        } else {
            &output.stdout
        };

        log_file
            .write_all(to_write)
            .await
            .expect("Failed to write the output to the log file");

        log_file
            .write_all("--- stderr ---\n".as_bytes())
            .await
            .expect("Failed to write the output to the log file");

        let log_stderr_value = log_stderr.lock().unwrap();
        let v;
        let to_write = if log_stderr_value.is_some() {
            debug!("test");
            v = log_stderr_value.clone().unwrap();
            v.as_bytes()
        } else {
            &output.stderr
        };

        log_file
            .write_all(to_write)
            .await
            .expect("Failed to write the output to the log file");

        Ok(())
    }

    async fn run(
        proton: &Proton,
        binary_path: PathBuf,
        custom_command: Option<String>,
    ) -> anyhow::Result<Result<Child, std::io::Error>> {
        let mut command: Vec<&str> = vec![];

        let proton_path = GameConfig::get_config_directory()
            .await
            .join("proton")
            .join("proton");

        command.push(proton.python.to_str().unwrap());
        command.push(proton_path.to_str().unwrap());
        command.push("run");
        command.push(binary_path.to_str().unwrap());

        let launch_option;
        if custom_command.is_some() {
            launch_option = custom_command.unwrap();
            debug!("Starting game with --options -> {}", launch_option.clone());
            let tokens: Vec<&str> = launch_option.split_whitespace().collect();

            // position of the %command%
            let index = if let Some(v) = tokens.iter().position(|&s| s == "%command%") {
                v
            } else {
                anyhow::bail!("You forget to put %command% in your custom launch command");
            };

            let start_command = tokens[0..index].to_vec();
            let mut end_command = tokens[(index + 1)..tokens.len()].to_vec();

            start_command.iter().rev().for_each(|str| {
                command.insert(0, str);
            });
            command.append(&mut end_command);
        } else {
            debug!("Starting game without --options");
        }

        debug!("Command preview -> {}", command.join(" "));

        Ok(Command::new(command[0])
            .args(&command[1..command.len()])
            .envs(proton.get_envs())
            .env("PROTON_LOG", "1")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn())
    }
}

fn notify_fonts_progress<P>(nbr: u64, max: u64, progress: &Option<Arc<P>>)
where
    P: Reporter + 'static,
{
    info!("{}", format!("{}/{} font installed", nbr, max));
    if let Some(p) = progress {
        p.progress(nbr);
    }
}
