#[cfg(target_os = "linux")]
fn get_binary() -> &'static str {
    "lucid"
}

// And this function only gets compiled if the target OS is *not* linux
#[cfg(not(target_os = "linux"))]
fn get_binary() -> &'static str {
    "lucid.exe"
}

// if cfg!(target_os = "linux") {
//     println!("Yes. It's definitely linux!");
// } else {
//     println!("Yes. It's definitely *not* linux!");
// }