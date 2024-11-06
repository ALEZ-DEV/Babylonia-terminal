use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Set to the config launch options to tinker the behavior of the game, you need to run this
    /// command one time to set your launch options to the configuration
    SetLaunchOptions { launch_options: String },

    /// Set the current game directory, this command will move all the current game file to the new one!
    SetGamePath { new_game_directory: String },
}
