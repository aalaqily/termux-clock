use clap::{Parser, Subcommand};

use termux_clock::{alarm::Alarm, timer::Timer};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
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

        #[arg(short, long, value_delimiter = ' ')]
        days: Option<Vec<u8>>,

        #[arg(short, long)]
        message: Option<String>,

        #[arg(short, long)]
        vibrate: bool,

        #[arg(short, long)]
        termux: bool,
    },
}

fn main() {
    let args = Args::parse();
    match args.cmd {
        Commands::Timer {
            hours,
            minutes,
            seconds,
            message,
            vibrate,
            termux,
        } => {
            let timer = Timer::from(hours, minutes, seconds, message, vibrate, termux);

            timer.set();
        }
        Commands::Alarm {
            hour,
            minutes,
            days,
            message,
            vibrate,
            termux,
        } => {
            let alarm = Alarm::from(hour, minutes, days, message, vibrate, termux);

            alarm.set();
        }
    }
}
