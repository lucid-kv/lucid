#[cfg(target_os = "linux")]
fn get_binary() -> &'static str {
    "./lucid"
}

#[cfg(not(target_os = "linux"))]
fn get_binary() -> &'static str {
    "lucid.exe"
}