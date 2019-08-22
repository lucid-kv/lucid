use colored::*;
use chrono::{DateTime, Utc};

pub enum LogLevel
{
    Success,
    Information,
    Warning,
    Error
}

pub trait Logger {
    fn log(&self, level: LogLevel, message: &str, _flag: &str);
}

impl<T> Logger for T {
    fn log(&self, level: LogLevel, message: &str, _flag: &str)
    {
        let now = Utc::now().format("%Y/%m/%d %H:%M:%S");
        match level {
            LogLevel::Success => {
                println!("{} [{}] lucid: {}", now, "SUCCESS", message);
            },
            LogLevel::Information => {
                println!("{} [{}]    lucid: {}", now, "INFO", message);
            },
            LogLevel::Warning => {
                eprintln!("{} [{}] lucid: {}", now, "WARNING", message);
            },
            LogLevel::Error => {
                eprintln!("{} [{}]   lucid: {}", now, "ERROR", message);
            }
        };
    }
}