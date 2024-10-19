use std::process::Command;

use crate::frontend::Frontend;

pub struct Termux;

impl Frontend for Termux {
    fn set_timer(length: Option<u32>, message: Option<String>) {
        let mut args: Vec<String> = vec!["-c".to_string()];
        match length {
            Some(l) => {
                args.push(format!("sleep {} &", l));
            }
            None => (),
        }
        match message {
            Some(m) => {
                &args[1].push_str(
                    format!(
                        "& termux-notification --title \"Termux Timer ({}s)\" --content \"{}\" &",
                        length.unwrap(),
                        m
                    )
                    .as_str(),
                );
            }
            None => {
                &args[1].push_str(format!("& termux-notification --title \"Termux Timer ({}s)\" --content \"Timer finished\" &", length.unwrap()).as_str());
            }
        }

        #[cfg(debug_assertions)]
        dbg!(&args);

        let mut command = Command::new("sh")
            .args(args)
            .spawn()
            .expect("Unable to set timer");

        #[cfg(debug_assertions)]
        {
            let status = command.wait().expect("Failed to wait for spawned process");
            dbg!(status);
        }
    }

    fn set_alarm(
        hour: Option<u32>,
        minute: Option<u32>,
        days: Option<Vec<u32>>,
        message: Option<String>,
        vibrate: Option<bool>,
    ) {
        let mut args: Vec<String> = vec!["-c".to_string(), String::new()];

        match message {
            Some(m) => &args[1].push_str(
                format!(
                    "echo \"termux-notification --title 'Termux Alarm' --content '{}' ",
                    m
                )
                .as_str(),
            ),
            None => &args[1].push_str("echo \"termux-notification --title 'Termux Alarm' "),
        };

        match vibrate {
            Some(v) => {
                if v {
                    &args[1].push_str("&& termux-vibrate");
                }
            }
            None => (),
        };

        match (hour, minute) {
            (Some(h), Some(m)) => &args[1].push_str(format!("\" | at {}:{}", h, m).as_str()),
            (Some(h), None) => &args[1].push_str(format!("\" | at {}:00", h).as_str()),
            (None, Some(m)) => &args[1].push_str(format!("\" | at 00:{}", m).as_str()),
            (None, None) => &args[1].push_str(format!("\" | at 00:00").as_str()),
        };

        #[cfg(debug_assertions)]
        dbg!(&args);

        #[cfg(debug_assertions)]
        dbg!(&args);

        let mut command = Command::new("sh")
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
}
