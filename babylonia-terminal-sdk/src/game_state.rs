use crate::{game_config::GameConfig, utils::kuro_prod_api::GameInfo};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GameState {
    ProtonNotInstalled,
    DXVKNotInstalled,
    FontNotInstalled,
    DependecieNotInstalled,
    GameNotInstalled,
    GameNeedUpdate,
    GameNotPatched,
    GameInstalled,
}

impl GameState {
    pub async fn get_current_state() -> anyhow::Result<Self> {
        let config = GameConfig::get_config().await;

        if !config.is_wine_installed {
            return Ok(GameState::ProtonNotInstalled);
        }

        if !config.is_dxvk_installed {
            return Ok(GameState::DXVKNotInstalled);
        }

        if !config.is_font_installed {
            return Ok(GameState::FontNotInstalled);
        }

        if !config.is_dependecies_installed {
            return Ok(GameState::DependecieNotInstalled);
        }

        if !config.is_game_installed {
            return Ok(GameState::GameNotInstalled);
        }

        if GameInfo::get_info().await?.need_update().await? {
            return Ok(GameState::GameNeedUpdate);
        }

        if !config.is_game_patched {
            return Ok(GameState::GameNotPatched);
        }

        Ok(GameState::GameInstalled)
    }
}
