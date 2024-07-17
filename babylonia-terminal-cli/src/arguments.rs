use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Pass launch options to tinker the behavior of the game, this parameter have priotiy over the
    /// --set-options param
    #[arg(long)]
    pub options: Option<String>,

    /// Set to the config launch options to tinker the behavior of the game, you need to run this
    /// command one time to set your launch options to the configuration
    #[arg(long)]
    pub set_options: Option<String>,
}
