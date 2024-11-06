use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<crate::commands::Commands>,

    /// Pass launch options to tinker the behavior of the game, this parameter have priority over the
    /// set-launch-options command
    #[arg(long)]
    pub options: Option<String>,

    /// Show the logs direcly to the stdout of your terminal
    #[arg(long, default_value = "false")]
    pub logs: bool,
}
