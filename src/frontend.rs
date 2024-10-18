pub trait Frontend {
    pub fn set_timer(length: Option<u32>, message: Option<String>);
    pub fn set_alarm(hour: Option<String>, minute: Option<String>, days: Option<[u32]>, message: Option<String>, vibrate: Option<bool>);
}
