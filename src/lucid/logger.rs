//use chrono::{DateTime, Utc};
//
//include!("../error.rs");
//
//// TODO: move in mod
//fn log_event(status: LucidError, message: String, method: String)
//{
//    let now: DateTime<Utc> = Utc::now();
//    display_error(status, format!("{} [INFO] lucid: [{}] Requested page: {}" , now.format("%Y/%m/%d %H:%M:%S"), method, message));
//}

use lucid::Lucid;

pub trait Logger {
    fn mdr(&self);
}

impl Logger for Lucid {
    fn mdr(&self)
    {
        println!("lol")
    }
}