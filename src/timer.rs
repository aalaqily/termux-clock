#[derive(Debug, PartialEq)]
pub struct Timer {
    pub length: Option<u32>,
    pub message: Option<String>,
    pub vibrate: bool,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            length: None,
            message: None,
            vibrate: false,
        }
    }

    pub fn hour(mut self, hour: u32) -> Self {
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
        let left = Timer {
            length: None,
            message: None,
            vibrate: false,
        };

        let right = Timer::new();

        assert_eq!(left, right);
    }

    #[test]
    fn test_hour() {
        let alarm = Timer::new().hour(6);
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
    fn test_message() {
        let alarm = Timer::new().message(String::from("Wake Up!"));
        assert_eq!(alarm.message, Some(String::from("Wake Up!")));
    }

    #[test]
    fn test_vibrate() {
        let alarm = Timer::new().vibrate(true);
        assert_eq!(alarm.vibrate, true);
    }
}
