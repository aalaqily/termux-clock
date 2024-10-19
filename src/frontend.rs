pub trait Frontend {
    fn set_timer(length: Option<u32>, message: Option<String>);
    fn set_alarm(
        hour: Option<u32>,
        minutes: Option<u32>,
        days: Option<Vec<u32>>,
        message: Option<String>,
        vibrate: Option<bool>,
    );
}
