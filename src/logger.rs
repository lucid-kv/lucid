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

pub trait Logger {
    fn log(&self);
}
