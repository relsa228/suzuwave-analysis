use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, disable_help_flag = true, disable_version_flag = true)]
pub struct Args {
    #[arg(short)]
    pub f: Option<String>,

    #[arg(short)]
    pub v: bool,
    #[arg(long)]
    pub version: bool,

    #[arg(short)]
    pub h: bool,
    #[arg(long)]
    pub help: bool,
}
