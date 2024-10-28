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
