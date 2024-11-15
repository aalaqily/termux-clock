use crate::wrappers::{
    am,
    termux,
};

#[derive(Debug, PartialEq)]
pub struct Timer {
    pub length: Option<u32>,
    pub message: Option<String>,
    pub vibrate: bool,
    pub termux: bool,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            length: None,
            message: None,
            vibrate: false,
            termux: false,
        }
    }

    pub fn from(hours: Option<u32>,
        minutes: Option<u32>,
        seconds: Option<u32>,
        message: Option<String>,
        vibrate: bool,
        termux: bool,
    ) -> Timer {
        let mut timer = Timer {
            length: None,
            message,
            vibrate,
            termux,
        };

        if let Some(h) = hours {
            timer = timer.hours(h);
        };

        if let Some(m) = minutes {
            timer = timer.minutes(m);
        };

        if let Some(s) = seconds {
            timer = timer.seconds(s);
        };

        timer
    }

    pub fn hours(mut self, hour: u32) -> Self {
        self.length = Some(self.length.map_or(hour * 3600, |l| l + hour * 3600));
        self
    }

    pub fn minutes(mut self, minutes: u32) -> Self {
        self.length = Some(self.length.map_or(minutes * 60, |l| l + minutes * 60));
        self
    }

    pub fn seconds(mut self, seconds: u32) -> Self {
        self.length = Some(self.length.map_or(seconds, |l| l + seconds));
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

    pub fn set(self) {
	let mut command = if self.termux {
	    termux::set_timer_command(self)
	}
	else {
	    am::set_timer_command(self)
	};

	#[cfg(debug_assertions)]
        {
            let args = &command.get_args().map(|a| a.to_str().unwrap()).collect::<Vec<&str>>();
            dbg!(args);
            let args_str = &args.join(" ");
            dbg!(args_str);
        }

        let output = command
            .output()
            .expect("Unable to set timer");

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
        let left = Timer {
            length: None,
            message: None,
            vibrate: false,
	    termux: false,
        };

        let right = Timer::new();

        assert_eq!(left, right);
    }

    #[test]
    fn test_hours() {
        let alarm = Timer::new().hours(6);
        assert_eq!(alarm.length, Some(21_600));
    }

    #[test]
    fn test_minutes() {
        let alarm = Timer::new().minutes(30);
        assert_eq!(alarm.length, Some(1_800));
    }

    #[test]
    fn test_seconds() {
        let alarm = Timer::new().seconds(600);
        assert_eq!(alarm.length, Some(600));
    }

    #[test]
    fn test_hours_minutes() {
	let alarm = Timer::new().hours(2).minutes(30);
	assert_eq!(alarm.length, Some(9_000));
    }

    #[test]
    fn test_hours_seconds() {
	let alarm = Timer::new().hours(2).seconds(50);
	assert_eq!(alarm.length, Some(7_250));
    }

    #[test]
    fn test_minutes_seconds() {
	let alarm = Timer::new().minutes(30).seconds(15);
	assert_eq!(alarm.length, Some(1_815));
    }

    #[test]
    fn test_hours_minutes_seconds() {
	let alarm = Timer::new().hours(2).minutes(30).seconds(10);
	assert_eq!(alarm.length, Some(9_010));
    }

    #[test]
    fn test_message() {
        let alarm = Timer::new().message("Wake Up!");
        assert_eq!(&alarm.message.unwrap(), "Wake Up!");
    }

    #[test]
    fn test_vibrate() {
        let alarm = Timer::new().vibrate(true);
        assert!(alarm.vibrate);
    }

    #[test]
    fn test_termux() {
	let alarm = Timer::new().termux(true);
	assert!(alarm.termux);
    }
}
