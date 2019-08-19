use chrono::{DateTime, Utc};

// TODO: move as mod
fn log_event(message: String)
{
    let now: DateTime<Utc> = Utc::now();
    println!("{} [INFO] lucid: Requested page: {}" , now.format("%Y/%m/%d %H:%M:%S"), message);
}