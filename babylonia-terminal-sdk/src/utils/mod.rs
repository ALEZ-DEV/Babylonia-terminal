pub mod fs;
pub mod github_requester;
pub mod kuro_prod_api;

pub fn get_game_name() -> String {
    concat!("P", "G", "R").to_string()
}

pub fn get_game_name_with_executable() -> String {
    format!("{}.exe", get_game_name())
}
