use std::process::Command;

use crate::frontend::Frontend;

pub struct Android;

impl Frontend for Android {
    fn set_timer(length: Option<u32>, message: Option<String>) {
        let mut args: Vec<String> = vec![
            "start".to_string(),
            "-a".to_string(),
            "android.intent.action.SET_TIMER".to_string(),
            "--ez".to_string(),
            "android.intent.extra.alarm.SKIP_UI".to_string(),
            "true".to_string(),
        ];

        match length {
            Some(l) => {
                args.push("--ei".to_string());
                args.push("android.intent.extra.alarm.LENGTH".to_string());
                args.push(l.to_string());
            }
            None => (),
        };

        match message {
            Some(m) => {
                args.push("--es".to_string());
                args.push("android.intent.extra.alarm.MESSAGE".to_string());
                args.push(m);
            }
            None => (),
        };

        #[cfg(debug_assertions)]
        {
            let args = &args.join(" ");
            dbg!(args);
        }

        let command = Command::new("am")
            .args(args)
            .output()
            .expect("Unable to set timer");

        #[cfg(debug_assertions)]
        {
            let status = command.status;
            let stdout = String::from_utf8(command.stdout).unwrap();
            let stderr = String::from_utf8(command.stderr).unwrap();
            dbg!(status);
            dbg!(stdout);
            dbg!(stderr);
        }
    }

    fn set_alarm(
        hour: Option<u32>,
        minutes: Option<u32>,
        days: Option<Vec<u32>>,
        message: Option<String>,
        vibrate: Option<bool>,
    ) {
        let mut args: Vec<String> = vec![
            "start".to_string(),
            "-a".to_string(),
            "android.intent.action.SET_ALARM".to_string(),
            "--ez".to_string(),
            "android.intent.extra.alarm.SKIP_UI".to_string(),
            "true".to_string(),
        ];

        match hour {
            Some(h) => {
                args.push("--ei".to_string());
                args.push("android.intent.extra.alarm.HOUR".to_string());
                args.push(h.to_string());
            }
            None => (),
        };

        match minutes {
            Some(m) => {
                args.push("--ei".to_string());
                args.push("android.intent.extra.alarm.MINUTES".to_string());
                args.push(m.to_string());
            }
            None => (),
        };

        match days {
            Some(d) => {
                args.push("--eial".to_string());
                args.push("android.intent.extra.alarm.DAYS".to_string());
                args.push(
                    d.iter()
                        .map(|n| n.to_string())
                        .collect::<Vec<String>>()
                        .join(","),
                );
            }
            None => (),
        };

        match message {
            Some(m) => {
                args.push("--es".to_string());
                args.push("android.intent.extra.alarm.MESSAGE".to_string());
                args.push(m);
            }
            None => (),
        };

        match vibrate {
            Some(v) => {
                args.push("--ez".to_string());
                args.push("android.intent.extra.alarm.VIBRATE".to_string());
                args.push(v.to_string());
            }
            None => (),
        };

        #[cfg(debug_assertions)]
        {
            let args = &args.join(" ");
            dbg!(args);
        }

        let command = Command::new("am")
            .args(args)
            .output()
            .expect("Unable to set alarm");

        #[cfg(debug_assertions)]
        {
            let status = command.status;
            let stdout = String::from_utf8(command.stdout).unwrap();
            let stderr = String::from_utf8(command.stderr).unwrap();
            dbg!(status);
            dbg!(stdout);
            dbg!(stderr);
        }
    }
}
