use crate::alarm::Alarm;
use crate::timer::Timer;
use std::process::Command;

pub fn set_timer_command(timer: Timer) -> Command {
    let mut args: Vec<String> = vec!["-c".to_string()];
    if let Some(l) = timer.length {
        args.push(format!("sleep {} &", l));
    };

    match timer.message {
        Some(m) => {
            &args[1].push_str(
                format!(
                    r#"& termux-notification --title "Termux Timer ({}s)" --content "{}" &"#,
                    timer.length.unwrap(),
                    m
                )
                .as_str(),
            );
        }
        None => {
            &args[1].push_str(format!(r#"& termux-notification --title "Termux Timer ({}s)" --content "Timer finished" &"#, timer.length.unwrap()).as_str());
        }
    };

    if timer.vibrate {
        &args[1].push_str("& termux-vibrate &");
    };

    let mut command = Command::new("sh");
    command.args(args);
    command
}

pub fn set_alarm_string(alarm: Alarm) -> String {
    let mut string = String::from("termux-notification --title 'Termux Alarm' ");

    if let Some(m) = alarm.message {
        string.push_str(format!("--content '{}' ", m).as_str())
    };

    if alarm.vibrate {
        string.push_str("&& termux-vibrate");
    };

    string
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsStr;

    #[test]
    fn test_set_timer_command() {
        let left_args: Vec<&OsStr> = [
            "-c",
            r#"sleep 60 && termux-notification --title "Termux Timer (60s)" --content "Wake up!" && termux-vibrate &"#,
        ]
        .iter()
        .map(|s| OsStr::new(s))
        .collect();

        let command = set_timer_command(
            Timer::new()
                .seconds(60)
                .message(String::from("Wake up!"))
                .vibrate(true),
        );

        let right_args = command.get_args().collect::<Vec<&OsStr>>();

        assert_eq!(left_args, right_args);
    }

    #[test]
    fn test_set_timer_command_vibrate_false() {
        let left_args: Vec<&OsStr> = [
            "-c",
            r#"sleep 60 && termux-notification --title "Termux Timer (60s)" --content "Wake up!" &"#,
        ]
        .iter()
        .map(|s| OsStr::new(s))
        .collect();

        let command = set_timer_command(Timer::new().seconds(60).message(String::from("Wake up!")));

        let right_args = command.get_args().collect::<Vec<&OsStr>>();

        assert_eq!(left_args, right_args);
    }

    #[test]
    fn test_set_timer_command_message_none() {
        let left_args: Vec<&OsStr> = [
            "-c",
            r#"sleep 60 && termux-notification --title "Termux Timer (60s)" --content "Timer finished" && termux-vibrate &"#,
        ]
        .iter()
        .map(|s| OsStr::new(s))
        .collect();

        let command = set_timer_command(Timer::new().seconds(60).vibrate(true));

        let right_args = command.get_args().collect::<Vec<&OsStr>>();

        assert_eq!(left_args, right_args);
    }

    #[test]
    fn test_set_alarm_string() {
        let left = String::from(
            "termux-notification --title 'Termux Alarm' --content 'Wake up!' && termux-vibrate",
        );

        let right = set_alarm_string(Alarm::new().message(String::from("Wake up!")).vibrate(true));

        assert_eq!(left, right);
    }

    #[test]
    fn test_set_alarm_string_message_none() {
        let left = String::from("termux-notification --title 'Termux Alarm' && termux-vibrate");

        let right = set_alarm_string(Alarm::new().vibrate(true));

        assert_eq!(left, right);
    }

    #[test]
    fn test_set_alarm_vibrate_false() {
        let left = String::from("termux-notification --title 'Termux Alarm' --content 'Wake up!' ");

        let right = set_alarm_string(Alarm::new().message(String::from("Wake up!")));

        assert_eq!(left, right);
    }
}
