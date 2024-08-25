use babylonia_terminal_sdk::game_state::GameState;

use crate::messages::{
    error::ReportError,
    game_state::{AskGameState, GameState as GameStateMessage, States},
};

impl GameStateMessage {
    fn from_sdk_state_to_msg_state(state: GameState) -> Self {
        let game_state = match state {
            GameState::ProtonNotInstalled => States::ProtonNotInstalled,
            GameState::DXVKNotInstalled => States::DxvkNotInstalled,
            GameState::FontNotInstalled => States::FontNotInstalled,
            GameState::DependecieNotInstalled => States::DependecieNotInstalled,
            GameState::GameNotInstalled => States::GameNotInstalled,
            GameState::GameNeedUpdate => States::GameNeedUpdate,
            GameState::GameNotPatched => States::GameNotPatched,
            GameState::GameInstalled => States::GameInstalled,
        };
        GameStateMessage {
            state: game_state.into(),
        }
    }
}

pub async fn listen_game_state() {
    let mut receiver = AskGameState::get_dart_signal_receiver().unwrap();
    while let Some(_) = receiver.recv().await {
        let result_state = GameState::get_current_state().await;
        match result_state {
            Ok(state) => GameStateMessage::from_sdk_state_to_msg_state(state).send_signal_to_dart(),
            Err(e) => ReportError {
                error_message: format!("When updating the game state : {}", e),
            }
            .send_signal_to_dart(),
        }
    }
}
