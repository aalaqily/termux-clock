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

    pub fn hour(&mut self, hour: u8) -> &mut Alarm {
	self.hour = hour;
	self
    }

    pub fn minutes(&mut self, minutes: u8) -> &mut Alarm {
	self.minutes = minutes;
	self
    }

    pub fn days(&mut self, days: Vec<u8>) -> &mut Alarm {
	self.days = Some(days);
	self
    }

    pub fn message(&mut self, message: String) -> &mut Alarm {
	self.message = Some(message);
	self
    }

    pub fn vibrate(&mut self, vibrate: bool) -> &mut Alarm {
	self.vibrate = vibrate;
	self
    }
}
