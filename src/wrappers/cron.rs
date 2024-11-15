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
    command.arg("-c").arg(format!(
        r#"(crontab -l; echo "{}") | crontab -"#,
        cron_entry
    ));
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
    command.arg("-c").arg(format!(
        r#"(crontab -l; echo "{}") | crontab -"#,
        cron_entry
    ));
    command
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsStr;
    use CronField;

    #[test]
    fn test_to_cron_string_value() {
        let left = String::from("40");
        let cron_field = CronField::Vector(vec![40]);
        let right = cron_field.to_cron_string();

        assert_eq!(left, right);
    }

    #[test]
    fn test_to_cron_string_values() {
        let left = String::from("1,2,3");
        let cron_field = CronField::Vector(vec![1, 2, 3]);
        let right = cron_field.to_cron_string();

        assert_eq!(left, right);
    }

    #[test]
    fn test_to_cron_string_all() {
        let left = String::from("*");
        let cron_field = CronField::All;
        let right = cron_field.to_cron_string();
        assert_eq!(left, right);
    }

    #[test]
    fn test_schedule_string_command() {
        let left = r#"(crontab -l; echo "40 13 * 1,2,3 * ls ~") | crontab -"#;
        let command = schedule_string_command(
            CronField::Vector(vec![40]),
            CronField::Vector(vec![13]),
            CronField::All,
            CronField::Vector(vec![1, 2, 3]),
            CronField::All,
            String::from("ls ~"),
        );
        let right = command.get_args().collect::<Vec<&OsStr>>()[1];

        assert_eq!(left, right);
    }

    #[test]
    fn test_schedule_alarm_command() {
        let left = r#"(crontab -l; echo "40 13 * * 1,2,3 termux-notification --title 'Termux Alarm' && termux-vibrate") | crontab -"#;
        let command = schedule_alarm_command(
            Alarm::new()
                .hour(13)
                .minutes(40)
                .days(vec![1, 2, 3])
                .vibrate(true),
        );
        let right = command.get_args().collect::<Vec<&OsStr>>()[1];

        assert_eq!(left, right);
    }
}
