use std::process::Command;

use crate::wrappers::{am, at, cron};

#[derive(Clone, Debug, PartialEq)]
pub struct Alarm {
    pub hour: Option<u8>,
    pub minutes: Option<u8>,
    pub days: Option<Vec<u8>>,
    pub message: Option<String>,
    pub vibrate: bool,
    pub termux: bool,
}

impl Alarm {
    pub fn new() -> Alarm {
        Alarm {
            hour: None,
            minutes: None,
            days: None,
            message: None,
            vibrate: false,
            termux: false,
        }
    }

    pub fn from(
        hour: Option<u8>,
        minutes: Option<u8>,
        days: Option<Vec<u8>>,
        message: Option<String>,
        vibrate: bool,
        termux: bool,
    ) -> Alarm {
        Alarm {
            hour,
            minutes,
            days,
            message,
            vibrate,
            termux,
        }
    }

    pub fn hour(mut self, hour: u8) -> Self {
        self.hour = Some(hour);
        self
    }

    pub fn minutes(mut self, minutes: u8) -> Self {
        self.minutes = Some(minutes);
        self
    }

    pub fn days(mut self, days: Vec<u8>) -> Self {
        self.days = Some(days);
        self
    }

    pub fn message(mut self, message: &str) -> Self {
        self.message = Some(String::from(message));
        self
    }

    pub fn vibrate(mut self, vibrate: bool) -> Self {
        self.vibrate = vibrate;
        self
    }

    pub fn termux(mut self, termux: bool) -> Self {
        self.termux = termux;
        self
    }

    pub fn set(mut self) {
        let mut command = if self.termux {
            match self.days {
                Some(d) => {
		    let check_crond = Command::new("sh")
			.args(["-c",
			       r#"ps -ef | grep -E "[0-9] crond""#])
			.status()
			.expect("Failed to check if crond is running");
		    if !check_crond.success() {
			Command.new("crond").output();
		    }
		    self.days = Some(d.iter().map(|x| x - 1 ).collect());
		    cron::schedule_alarm_command(self)
		},
                None => {
		    let check_crond = Command::new("sh")
			.args(["-c",
			       r#"ps -ef | grep -E "[0-9] atd""#])
			.status()
			.expect("Failed to check if atd is running");
		    if !check_crond.success() {
			Command.new("atd").output();
		    }
		    at::schedule_alarm_command(self)
		},
            }
        } else {
            am::set_alarm_command(self)
        };

        #[cfg(debug_assertions)]
        {
            let args = command
                .get_args()
                .map(|a| a.to_str().unwrap())
                .collect::<Vec<&str>>();
            dbg!(&args);
            let args_str = &args.join(" ");
            dbg!(args_str);
        }

        let output = command.output().expect("Unable to set alarm");

        #[cfg(debug_assertions)]
        {
            let status = output.status;
            let stdout = String::from_utf8(output.stdout).unwrap();
            let stderr = String::from_utf8(output.stderr).unwrap();
            dbg!(status);
            dbg!(stdout);
            dbg!(stderr);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let left = Alarm {
            hour: None,
            minutes: None,
            days: None,
            message: None,
            vibrate: false,
            termux: false,
        };

        let right = Alarm::new();

        assert_eq!(left, right);
    }

    #[test]
    fn test_from() {
        let alarm = Alarm::from(
            Some(6),
            Some(30),
            Some(vec![1, 2, 3]),
            Some(String::from("Wake Up!")),
            true,
            true,
        );

        assert_eq!(alarm.hour, Some(6));
        assert_eq!(alarm.minutes, Some(30));
        assert_eq!(alarm.days, Some(vec![1, 2, 3]));
        assert_eq!(&alarm.message.unwrap(), "Wake Up!");
        assert_eq!(alarm.vibrate, true);
        assert_eq!(alarm.termux, true);
    }

    #[test]
    fn test_hour() {
        let alarm = Alarm::new().hour(6);
        assert_eq!(alarm.hour, Some(6));
    }

    #[test]
    fn test_minutes() {
        let alarm = Alarm::new().minutes(30);
        assert_eq!(alarm.minutes, Some(30));
    }

    #[test]
    fn test_days() {
        let alarm = Alarm::new().days(vec![1, 2, 3]);
        assert_eq!(alarm.days, Some(vec![1, 2, 3]));
    }

    #[test]
    fn test_message() {
        let alarm = Alarm::new().message("Wake Up!");
        assert_eq!(&alarm.message.unwrap(), "Wake Up!");
    }

    #[test]
    fn test_vibrate() {
        let alarm = Alarm::new().vibrate(true);
        assert_eq!(alarm.vibrate, true);
    }

    #[test]
    fn test_termux() {
        let alarm = Alarm::new().termux(true);
        assert_eq!(alarm.termux, true);
    }
}
