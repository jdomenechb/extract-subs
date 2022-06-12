use colored::Colorize;

pub fn format_error(message: &str) -> String {
    format!("ERROR: {}", message).red().to_string()
}

pub fn format_warning(message: &str) -> String {
    format!("WARNING: {}", message).yellow().to_string()
}

pub fn format_success(message: &str) -> String {
    format!("{}", message).green().to_string()
}

pub fn print_error(message: String) {
    println!("{}", format_error(message.as_str()));
}

pub fn print_warning(message: String) {
    println!("{}", format_warning(message.as_str()));
}

pub fn print_success(message: String) {
    println!("{}", format_success(message.as_str()));
}
