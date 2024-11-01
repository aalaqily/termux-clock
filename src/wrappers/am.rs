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
        args.push("--ei".to_string());
        args.push("android.intent.extra.alarm.LENGTH".to_string());
        args.push(l.to_string());
    };

    if let Some(m) = timer.message {
        args.push("--es".to_string());
        args.push("android.intent.extra.alarm.MESSAGE".to_string());
        args.push(m);
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
        args.push("--eial".to_string());
        args.push("android.intent.extra.alarm.DAYS".to_string());
        args.push(
            d.iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(","),
        );
    };

    if let Some(m) = alarm.message {
        args.push("--es".to_string());
        args.push("android.intent.extra.alarm.MESSAGE".to_string());
        args.push(m);
    };

    if !alarm.vibrate {
        args.push("--ez".to_string());
        args.push("android.intent.extra.alarm.VIBRATE".to_string());
        args.push(false.to_string());
    };

    let mut command = Command::new("am");
    command.args(args);
    command
}
