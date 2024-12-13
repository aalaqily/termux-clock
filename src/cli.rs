use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author = "Ahmad Asaad <ahmadasaadh@gmail.com>", version, about = "Termux tool to set alarms and timers headlessly", long_about = "Termux tool to set alarms and timers headlessly. Licensed under MIT license")]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    #[command(author = "Ahmad Asaad <ahmadasaadh@gmail.com>", about = "Set timer", long_about = None)]
    Timer {
        #[arg(short = 'H', long, help = "add hours to timer length")]
        hours: Option<u32>,

        #[arg(short = 'M', long, help = "add minutes to timer length")]
        minutes: Option<u32>,

        #[arg(short = 'S', long, help = "add seconds to timer length")]
        seconds: Option<u32>,

        #[arg(short, long, help = "timer message")]
        message: Option<String>,

        #[arg(short, long, help = "enable vibration", long_help = "enable vibration. not available in android timers, so use it with `--termux` option")]
        vibrate: bool,

        #[arg(short, long, help = "set timer in termux instead of android alarm clock")]
        termux: bool,
    },

    #[command(author = "Ahmad Asaad <ahmadasaadh@gmail.com>", about = "Set alarm", long_about = None)]
    Alarm {
        #[arg(short = 'H', long, help = "alarm hour")]
        hour: Option<u8>,

        #[arg(short = 'M', long, help = "alarm extra minutes")]
        minutes: Option<u8>,

        #[arg(short, long, value_delimiter = ',', help = "days to recurr the alarm, denoted by comma-seperated numbers, where each number corresponds to a weekday, starting from sunday", long_help = "days to recurr the alarm, denoted by comma-seperated numbers (e. g. `1,2,3`), where each number corresponds to a weekday, starting from sunday (i. e. sunday is `1`, monday is `2` and so on and so forth)")]
        days: Option<Vec<u8>>,

        #[arg(short, long, help = "alarm message")]
        message: Option<String>,

        #[arg(short, long, help = "enable vibration")]
        vibrate: bool,

        #[arg(short, long, help = "set alarm in termux instead of android alarm clock")]
        termux: bool,

        #[arg(long, help = "set alarm hour in pm instead of am")]
        pm: bool,
    },
}
