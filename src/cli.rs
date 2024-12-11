use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Timer {
        #[arg(short = 'H', long)]
        hours: Option<u32>,

        #[arg(short = 'M', long)]
        minutes: Option<u32>,

        #[arg(short = 'S', long)]
        seconds: Option<u32>,

        #[arg(short, long)]
        message: Option<String>,

        #[arg(short, long)]
        vibrate: bool,

        #[arg(short, long)]
        termux: bool,
    },

    Alarm {
        #[arg(short = 'H', long)]
        hour: Option<u8>,

        #[arg(short = 'M', long)]
        minutes: Option<u8>,

        #[arg(short, long, value_delimiter = ',')]
        days: Option<Vec<u8>>,

        #[arg(short, long)]
        message: Option<String>,

        #[arg(short, long)]
        vibrate: bool,

        #[arg(short, long)]
        termux: bool,

        #[arg(long)]
        pm: bool,
    },
}
