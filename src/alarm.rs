#[derive(Debug, PartialEq)]
pub struct Alarm {
    hour: u8,
    minutes: u8,
    days: Option<Vec<u8>>,
    message: Option<String>,
    vibrate: bool,
}

impl Alarm {
    pub fn new() -> Alarm {
	Alarm {
	    hour: 0,
	    minutes: 0,
	    days: None,
	    message: None,
	    vibrate: false,
	}
    }

    pub fn hour(mut self, hour: u8) -> Self {
	self.hour = hour;
	self
    }

    pub fn minutes(mut self, minutes: u8) -> Self {
	self.minutes = minutes;
	self
    }

    pub fn days(mut self, days: Vec<u8>) -> Self {
	self.days = Some(days);
	self
    }

    pub fn message(mut self, message: String) -> Self {
	self.message = Some(message);
	self
    }

    pub fn vibrate(mut self, vibrate: bool) -> Self {
	self.vibrate = vibrate;
	self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
	let left = Alarm {
	    hour: 0,
	    minutes: 0,
	    days: None,
	    message: None,
	    vibrate: false,
	};

	let right = Alarm::new();

	assert_eq!(left, right);
    }

    #[test]
    fn test_hour() {
	let alarm = Alarm::new().hour(6);
	assert_eq!(alarm.hour, 6);
    }

    #[test]
    fn test_minutes() {
	let alarm = Alarm::new().minutes(30);
	assert_eq!(alarm.minutes, 30);
    }

    #[test]
    fn test_days() {
	let alarm = Alarm::new().days(vec![1, 2, 3]);
	assert_eq!(alarm.days, Some(vec![1, 2, 3]));
    }

    #[test]
    fn test_message() {
	let alarm = Alarm::new().message(String::from("Wake Up!"));
	assert_eq!(alarm.message, Some(String::from("Wake Up!")));
    }

    #[test]
    fn test_vibrate() {
	let alarm = Alarm::new().vibrate(true);
	assert_eq!(alarm.vibrate, true);
    }
}
