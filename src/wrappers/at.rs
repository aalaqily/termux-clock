use crate::alarm::Alarm;
use crate::wrappers::termux::set_alarm_string;
use std::process::Command;

pub fn schedule_string_command(string: String, hour: u8, minute: u8) -> Command {
    let mut command = Command::new("sh");
    command
    .arg("-c")
    .arg(format!(r#"echo "{}" | at {}:{}"#, string, hour, minute));
    command
}

pub fn schedule_alarm_command(alarm: Alarm) -> Command {
    let mut command = Command::new("sh");
    command
    .arg("-c")
    .arg(format!(r#"echo "{}" | at {}:{}"#, set_alarm_string(alarm.clone()), if let Some(h) = alarm.hour { h } else { 0 }, if let Some(m) = alarm.minutes { m } else { 0 }));
    command
}



