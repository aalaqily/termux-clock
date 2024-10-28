#[derive(Debug, PartialEq)]
pub struct Timer {
    length: u32,
    message: Option<String>,
    vibrate: bool,
}

impl Timer {
    pub fn new() -> Timer {
	Timer {
	    length: 0,
	    message: None,
	    vibrate: false,
	}
    }

    pub fn hour(mut self, hour: u32) -> Self {
	self.length += hour * 3600;
	self
    }

    pub fn minutes(mut self, minutes: u32) -> Self {
	self.length += minutes * 60;
	self
    }

    pub fn seconds(mut self, seconds: u32) -> Self {
	self.length += seconds;
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
