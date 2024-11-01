use crate::alarm::Alarm;
use crate::timer::Timer;
use std::process::Command;

fn set_timer_command(timer: Timer) -> Command {
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

    let mut command = Command::new("sh");
    command.args(args);
    command
}

fn set_alarm_string(alarm: Alarm) -> String {
    let mut string = String::from("termux-notification --title 'Termux Alarm' ");

    if let Some(m) = alarm.message {
        string.push_str(format!("--content '{}' ", m).as_str())
    };

    if alarm.vibrate {
        string.push_str("&& termux-vibrate");
    };

    string
}
