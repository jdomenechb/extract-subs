use colored::Colorize;

pub fn format_error(message: &str) -> String {
    format!("ERROR: {}", message.red())
}

pub fn print_warning(message: String) {
    println!("WARNING: {}", message.yellow());
}

pub fn print_error(message: String) {
    println!("ERROR: {}", message.red());
}

pub fn print_success(message: String) {
    println!("{}", message.green());
}
