pub trait Frontend {
    fn set_timer(length: Option<&str>, message: Option<&str>);
    fn set_alarm(
        hour: Option<&str>,
        minute: Option<&str>,
        days: Option<&str>,
        message: Option<&str>,
        vibrate: Option<&str>,
    );
}
