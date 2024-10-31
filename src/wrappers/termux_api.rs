fn termux_notification(title: Option<String>, content: Option<String>) -> String {
    let mut command = vec![String::from("termux-notification")];

    if let Some(title) = title {
	command.push(format!("--title \"{}\"", title));
    }

    if let Some(content) = content {
	command.push(format!("--content \"{}\"", content));
    }
    
    command.join(" ")
}
