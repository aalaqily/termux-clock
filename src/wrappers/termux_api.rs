pub fn termux_notification(title: Option<String>, content: Option<String>) -> String {
    let mut command = vec![String::from("termux-notification")];

    if let Some(title) = title {
	command.push(format!(r#"--title "{}""#, title));
    }

    if let Some(content) = content {
	command.push(format!(r#"--content "{}""#, content));
    }
    
    command.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_termux_notification() {
	let left = String::from(r#"termux-notification --title "Termux Clock" --content "Wake up!""#);
	let right = termux_notification(Some(String::from("Termux Clock")), Some(String::from("Wake up!")));
	assert_eq!(left, right);
    }

    #[test]
    fn test_termux_notification_with_title() {
	let left = String::from(r#"termux-notification --title "Termux Clock""#);
	let right = termux_notification(Some(String::from("Termux Clock")), None);
	assert_eq!(left, right);
    }

    #[test]
    fn test_termux_notification_with_content() {
	let left = String::from(r#"termux-notification --content "Wake up!""#);
	let right = termux_notification(None, Some(String::from("Wake up!")));
	assert_eq!(left, right);
    }
}
