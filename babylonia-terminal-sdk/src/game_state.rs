use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::PathBuf,
};
use tokio::{
    fs::{read_to_string, File},
    io::{AsyncReadExt, AsyncWriteExt},
};
//use wincompatlib::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub enum GameState {
    WineNotInstalled,
    DXVKNotInstalled,
    FontNotInstalled,
    DependecieNotInstalled, // that's just the missing dll to install
    LauncherNotInstalled,
    GameNotInstalled,
    InstallingGame,
    GameInstalled,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameConfig {
    pub dedicated_wine: bool,
    pub is_wine_installed: bool,
    pub wine_path: Option<String>,
    pub is_dxvk_installed: bool,
    pub is_font_installed: bool,
    pub is_dependecies_installed: bool,
    pub is_game_installed: bool, // will be remove, just for test purpose
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig {
            dedicated_wine: true,
            is_wine_installed: false,
            wine_path: None,
            is_dxvk_installed: false,
            is_font_installed: false,
            is_dependecies_installed: false,
            is_game_installed: false,
        }
    }
}

impl GameState {
    pub fn get_config_directory() -> PathBuf {
        dirs::home_dir().unwrap().join(".babylonia-terminal")
    }

    fn get_config_file_path() -> PathBuf {
        GameState::get_config_directory().join("babylonia-terminal-config")
    }

    async fn try_get_config_file() -> anyhow::Result<File> {
        let _ = tokio::fs::create_dir(GameState::get_config_directory()).await;

        Ok(tokio::fs::File::create(GameState::get_config_file_path()).await?)
    }

    pub async fn save_config(config: GameConfig) -> anyhow::Result<()> {
        let mut file = GameState::try_get_config_file().await?;
        let content = serde_json::to_string(&config)?;
        file.write_all(content.as_bytes()).await?;

        Ok(())
    }

    pub async fn get_config() -> anyhow::Result<GameConfig> {
        let content = match read_to_string(GameState::get_config_file_path()).await {
            Err(_) => return Ok(GameConfig::default()),
            Ok(c) => c,
        };
        if let Ok(config) = serde_json::from_str::<GameConfig>(&content) {
            Ok(config)
        } else {
            Ok(GameConfig::default())
        }
    }

    pub async fn get_current_state() -> Self {
        let config = GameState::get_config().await.unwrap();

        if !config.is_wine_installed {
            return GameState::WineNotInstalled;
        }

        if !config.is_dxvk_installed {
            return GameState::DXVKNotInstalled;
        }

        if !config.is_font_installed {
            return GameState::FontNotInstalled;
        }

        if !config.is_dependecies_installed {
            return GameState::DependecieNotInstalled;
        }

        if !config.is_game_installed {
            return GameState::GameNotInstalled;
        }

        GameState::GameInstalled
    }
}
