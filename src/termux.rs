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
        todo!();
    }
}
