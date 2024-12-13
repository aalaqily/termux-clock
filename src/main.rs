use clap::Parser;

use termux_clock::{alarm::Alarm, timer::Timer, cli::{Args, Commands}};

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
            mut hour,
            minutes,
            days,
            message,
            vibrate,
            termux,
	    pm,
        } => {
	    if let None = hour {
		panic!("fatal: hour is required");
	    }
	    if pm {
		hour = hour.map(|v| v + 12);
	    }
            let alarm = Alarm::from(hour, minutes, days, message, vibrate, termux);

            alarm.set();
        }
    }
}
