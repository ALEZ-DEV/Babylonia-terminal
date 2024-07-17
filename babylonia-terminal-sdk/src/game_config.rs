use std::path::PathBuf;

use dirs::home_dir;
use serde::{Deserialize, Serialize};
use tokio::{
    fs::{create_dir_all, read_to_string, File},
    io::AsyncWriteExt,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameConfig {
    pub config_dir: PathBuf,
    pub is_wine_installed: bool,
    pub is_dxvk_installed: bool,
    pub is_font_installed: bool,
    pub is_dependecies_installed: bool,
    pub game_dir: Option<PathBuf>,
    pub is_game_installed: bool,
    pub is_game_patched: bool,
    pub launch_options: Option<String>,
}

impl GameConfig {
    pub async fn get_config_directory() -> PathBuf {
        let path = home_dir().unwrap().join(".babylonia-terminal"); // I will try to change that to a dynamic one if people want to change the config dir

        let _ = create_dir_all(path.clone()).await;

        path
    }

    async fn get_config_file_path() -> PathBuf {
        Self::get_config_directory()
            .await
            .join("babylonia-terminal-config")
    }

    pub async fn set_game_dir(path: Option<PathBuf>) -> anyhow::Result<()> {
        let mut config = Self::get_config().await;
        config.game_dir = path;
        Self::save_config(config).await?;
        Ok(())
    }

    pub async fn get_game_dir() -> Option<PathBuf> {
        Self::get_config().await.game_dir
    }

    async fn try_get_config_file() -> anyhow::Result<File> {
        let _ = tokio::fs::create_dir(Self::get_config_directory().await).await;

        Ok(tokio::fs::File::create(Self::get_config_file_path().await).await?)
    }

    pub async fn save_config(config: Self) -> anyhow::Result<()> {
        let mut file = Self::try_get_config_file().await?;
        let content = serde_json::to_string(&config)?;
        file.write_all(content.as_bytes()).await?;

        Ok(())
    }

    pub async fn get_config() -> Self {
        let content = match read_to_string(Self::get_config_file_path().await).await {
            Err(_) => return Self::default(),
            Ok(c) => c,
        };
        match serde_json::from_str::<Self>(&content) {
            Ok(config) => return config,
            Err(_) => return Self::default(),
        }
    }

    pub async fn set_launch_options(command: String) -> anyhow::Result<()> {
        let mut config = Self::get_config().await;
        config.launch_options = Some(command);
        Self::save_config(config).await?;
        Ok(())
    }

    pub async fn get_launch_options() -> anyhow::Result<Option<String>> {
        Ok(Self::get_config().await.launch_options)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameConfigPath {
    path: PathBuf,
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig {
            config_dir: dirs::home_dir().unwrap().join(".babylonia-terminal"),
            is_wine_installed: false,
            is_dxvk_installed: false,
            is_font_installed: false,
            is_dependecies_installed: false,
            game_dir: None,
            is_game_installed: false,
            is_game_patched: false,
            launch_options: None,
        }
    }
}
