//use colored::*;
use chrono::Utc;

include!("crossplatform.rs");

pub enum LogLevel
{
    Success,
    Information,
    Warning,
    Error
}

pub fn print(level: &LogLevel, message: &str) {
    let now = Utc::now().format("%Y/%m/%d %H:%M:%S");
    match level {
        LogLevel::Success => {
            println!("{} [{}] {}: {}", now, "SUCCESS", get_binary(), message);
        },
        LogLevel::Information => {
            println!("{} [{}] {}: {}", now, "INFO", get_binary(), message);
        },
        LogLevel::Warning => {
            eprintln!("{} [{}] {}: {}", now, "WARNING", get_binary(), message);
        },
        LogLevel::Error => {
            eprintln!("{} [{}] {}: {}", now, "ERROR", get_binary(), message);
        }
    };
}

pub trait Logger {
    fn log(&self, level: LogLevel, message: &str, err: Option<&str>);
}

impl<T> Logger for T {
    fn log(&self, level: LogLevel, message: &str, err: Option<&str>)
    {
        print(&level, &message);
        if err.is_some() {
            print(&level, err.unwrap());
        }
    }
}