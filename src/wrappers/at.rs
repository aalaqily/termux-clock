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

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsStr;

    #[test]
    fn test_schedule_string_command() {
        let left = r#"echo "ls ~" | at 6:30"#;
        
        let command = schedule_string_command(String::from("ls ~"), 6, 30);

        let right = command.get_args().collect::<Vec<&OsStr>>()[1];

        assert_eq!(left, right);
    }

    #[test]
    fn test_schedule_alarm_command() {
        let left = r#"echo "termux-notification --title 'Termux Alarm' --content 'Wake up!' && termux-vibrate" | at 6:30"#;
        
        let command = schedule_alarm_command(Alarm::new().hour(6).minutes(30).message(String::from("Wake up!")).vibrate(true));

        let right = command.get_args().collect::<Vec<&OsStr>>()[1];

        assert_eq!(left, right);
    }

    #[test]
    fn test_schedule_alarm_command_minutes_none() {
        let left = r#"echo "termux-notification --title 'Termux Alarm' --content 'Wake up!' && termux-vibrate" | at 6:0"#;
        
        let command = schedule_alarm_command(Alarm::new().hour(6).message(String::from("Wake up!")).vibrate(true));

        let right = command.get_args().collect::<Vec<&OsStr>>()[1];

        assert_eq!(left, right);
    }

    #[test]
    fn test_schedule_alarm_command_hour_none_minutes_none() {
        let left = r#"echo "termux-notification --title 'Termux Alarm' --content 'Wake up!' && termux-vibrate" | at 0:0"#;
        
        let command = schedule_alarm_command(Alarm::new().message(String::from("Wake up!")).vibrate(true));

        let right = command.get_args().collect::<Vec<&OsStr>>()[1];

        assert_eq!(left, right);
    }
}