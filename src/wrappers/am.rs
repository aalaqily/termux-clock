use crate::alarm::Alarm;
use crate::timer::Timer;
use std::process::Command;

fn set_timer(timer: Timer) -> Command {
    let mut args: Vec<String> = vec![
        "start".to_string(),
        "-a".to_string(),
        "android.intent.action.SET_TIMER".to_string(),
        "--ez".to_string(),
        "android.intent.extra.alarm.SKIP_UI".to_string(),
        "true".to_string(),
    ];

    if let Some(l) = timer.length {
        args.extend(vec![
            "--ei".to_string(),
            "android.intent.extra.alarm.LENGTH".to_string(),
            l.to_string(),
        ]);
    };

    if let Some(m) = timer.message {
        args.extend(vec![
            "--es".to_string(),
            "android.intent.extra.alarm.MESSAGE".to_string(),
            m,
        ])
    };

    let mut command = Command::new("am");
    command.args(args);
    command
}

fn set_alarm(alarm: Alarm) -> Command {
    let mut args: Vec<String> = vec![
        "start".to_string(),
        "-a".to_string(),
        "android.intent.action.SET_ALARM".to_string(),
        "--ez".to_string(),
        "android.intent.extra.alarm.SKIP_UI".to_string(),
        "true".to_string(),
    ];

    if let Some(h) = alarm.hour {
        args.extend(vec![
            "--ei".to_string(),
            "android.intent.extra.alarm.HOUR".to_string(),
            h.to_string(),
            "--ei".to_string(),
            "android.intent.extra.alarm.MINUTES".to_string(),
            alarm.minutes.unwrap_or(0).to_string(),
        ]);
    }

    if let Some(d) = alarm.days {
        args.extend(vec![
            "--eial".to_string(),
            "android.intent.extra.alarm.DAYS".to_string(),
            d.iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(","),
        ])
    };

    if let Some(m) = alarm.message {
        args.extend(vec![
            "--es".to_string(),
            "android.intent.extra.alarm.MESSAGE".to_string(),
            m,
        ])
    };

    if !alarm.vibrate {
        args.extend(vec![
            "--ez".to_string(),
            "android.intent.extra.alarm.VIBRATE".to_string(),
            false.to_string(),
        ])
    };

    let mut command = Command::new("am");
    command.args(args);
    command
}

#[cfg(test)]
mod tests {
    use std::ffi::OsStr;
    use super::*;

    #[test]
    fn test_set_timer() {
	let left_args: Vec<&OsStr> = ["start", "-a", "android.intent.action.SET_TIMER", "--ez", "android.intent.extra.alarm.SKIP_UI", "true", "--ei", "android.intent.extra.alarm.LENGTH", "60", "--es", "android.intent.extra.alarm.MESSAGE", "Wake up!"].iter().map(|s| OsStr::new(s)).collect();
	let command = set_timer(Timer::new().seconds(60).message(String::from("Wake up!")));
	
	assert_eq!(left_args, command.get_args().collect::<Vec<&OsStr>>());
	//assert_eq!();
    }
}
