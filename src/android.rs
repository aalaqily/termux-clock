use std::process::Command;

use crate::frontend::Frontend;

pub struct Android;

impl Frontend for Android {
    fn set_timer(length: Option<&str>, message: Option<&str>) {
        let mut args: Vec<&str> = vec![
            "start",
            "-a",
            "android.intent.action.SET_TIMER",
            "--ez",
            "android.intent.extra.alarm.SKIP_UI",
            "true",
        ];

        match length {
            Some(l) => {
                args.push("--ei");
                args.push("android.intent.extra.alarm.LENGTH");
                args.push(l);
            }
            None => (),
        };
	
        match message {
            Some(m) => {
                args.push("--es");
                args.push("android.intent.extra.alarm.MESSAGE");
                args.push(m);
            }
            None => (),
        };

        #[cfg(debug_assertions)]
        {
            println!("\x1b[1;34mcommand:\x1b[0m");
            for arg in (&args).iter() {
                print!("{} ", arg);
            }
            println!();
        }

        let command = Command::new("am")
            .args(args)
            .output()
            .expect("Unable to set timer");

        #[cfg(debug_assertions)]
        println!("\x1b[1;34mstatus:\x1b[0m\n{}\n\x1b[1;34mstdout:\x1b[0m\n{}\n\x1b[1;34mstderr:\x1b[0m\n{}", command.status, String::from_utf8(command.stdout).unwrap(), String::from_utf8(command.stderr).unwrap());
    }

    fn set_alarm(
        hour: Option<&str>,
        minutes: Option<&str>,
        days: Option<&str>,
        message: Option<&str>,
        vibrate: Option<&str>,
    ) {
        let mut args: Vec<&str> = vec![
            "start",
            "-a",
            "android.intent.action.SET_ALARM",
            "--ez",
            "android.intent.extra.alarm.SKIP_UI",
            "true",
        ];

        match hour {
            Some(h) => {
                args.push("--ei");
                args.push("android.intent.extra.alarm.HOUR");
                args.push(h);
            }
            None => (),
        };
	
        match minutes {
            Some(m) => {
                args.push("--ei");
                args.push("android.intent.extra.alarm.MINUTES");
                args.push(m);
            }
            None => (),
        };

        match days {
            Some(d) => {
                args.push("--eial");
                args.push("android.intent.extra.alarm.DAYS");
                args.push(d);
            }
            None => (),
        };

        match message {
            Some(m) => {
                args.push("--es");
                args.push("android.intent.extra.alarm.MESSAGE");
                args.push(m);
            }
            None => (),
        };

        match vibrate {
            Some(v) => {
                args.push("--ez");
                args.push("android.intent.extra.alarm.VIBRATE");
                args.push(v);
            }
            None => (),
        };

        #[cfg(debug_assertions)]
        {
            println!("\x1b[1;34mcommand:\x1b[0m");
            for arg in (&args).iter() {
                print!("{} ", arg);
            }
            println!();
        }

        let command = Command::new("am")
            .args(args)
            .output()
            .expect("Unable to set alarm");

        #[cfg(debug_assertions)]
        println!("\x1b[1;34mstatus:\x1b[0m\n{}\n\x1b[1;34mstdout:\x1b[0m\n{}\n\x1b[1;34mstderr:\x1b[0m\n{}", command.status, String::from_utf8(command.stdout).unwrap(), String::from_utf8(command.stderr).unwrap());
    }
}
