use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Pass launch options to tink the behavior of the game
    #[arg(long)]
    pub options: Option<String>,
}
