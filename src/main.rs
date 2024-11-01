mod android;
mod frontend;
mod termux;

use android::*;
use frontend::*;
use termux::*;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Timer {
        #[arg(short, long)]
        length: Option<u32>,

        #[arg(short, long)]
        message: Option<String>,

        #[arg(short, long)]
        termux: bool,
    },

    Alarm {
        #[arg(short = 'H', long)]
        hour: Option<u32>,

        #[arg(short = 'M', long)]
        minutes: Option<u32>,

        #[arg(short, long, value_delimiter = ' ')]
        days: Option<Vec<u32>>,

        #[arg(short, long)]
        message: Option<String>,

        #[arg(short, long)]
        vibrate: Option<bool>,

        #[arg(short, long)]
        termux: bool,
    },
}

fn main() {
    let args = Args::parse();
    match args.cmd {
        Commands::Timer {
            length,
            message,
            termux,
        } => {
            if termux {
                Termux::set_timer(length, message)
            } else {
                Android::set_timer(length, message)
            }
        }
        Commands::Alarm {
            hour,
            minutes,
            days,
            message,
            vibrate,
            termux,
        } => {
            if termux {
                Termux::set_alarm(hour, minutes, days, message, vibrate)
            } else {
                Android::set_alarm(hour, minutes, days, message, vibrate)
            }
        }
        _ => (),
    }
}
