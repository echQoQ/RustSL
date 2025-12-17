#[cfg(feature = "debug")]
use colored::*;

#[cfg(feature = "debug")]
pub fn print_error(_prefix: &str, _error: &dyn std::fmt::Display) {
    println!("{} {}", _prefix.red(), _error);
}

#[cfg(feature = "debug")]
pub fn print_message(msg: &str) {
    println!("{}",msg.green());
}