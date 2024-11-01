use crate::alarm::Alarm;
use crate::wrappers::termux::set_alarm_string;
use std::process::Command;

pub enum CronField {
    Vector(Vec<u8>),
    All,
}

impl CronField {
    fn to_cron_string(&self) -> String {
        match self {
            CronField::Vector(v) => v
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(","),
            CronField::All => "*".to_string(),
        }
    }
}

pub fn schedule_string_command(
    minute: CronField,
    hour: CronField,
    month_day: CronField,
    month: CronField,
    week_day: CronField,
    string: String,
) -> Command {
    let cron_entry = format!(
        r#"{} {} {} {} {} {}"#,
        minute.to_cron_string(),
        hour.to_cron_string(),
        month_day.to_cron_string(),
        month.to_cron_string(),
        week_day.to_cron_string(),
        string
    );

    let mut command = Command::new("sh");
    command
        .arg("-c")
        .arg(format!("(crontab -l; echo '{}') | crontab -", cron_entry));
    command
}
pub fn schedule_alarm_command(alarm: Alarm) -> Command {
    let hour = match alarm.hour {
        Some(h) => CronField::Vector(vec![h]),
        None => CronField::Vector(vec![0]),
    };

    let minute = match alarm.minutes {
        Some(m) => CronField::Vector(vec![m]),
        None => CronField::Vector(vec![0]),
    };

    let week_day = match alarm.days.clone() {
        Some(v) => CronField::Vector(v),
        None => CronField::All,
    };

    let cron_entry = format!(
        r#"{} {} * * {} {}"#,
        minute.to_cron_string(),
        hour.to_cron_string(),
        week_day.to_cron_string(),
        set_alarm_string(alarm),
    );

    let mut command = Command::new("sh");
    command
        .arg("-c")
        .arg(format!("(crontab -l; echo '{}') | crontab -", cron_entry));
    command
}
