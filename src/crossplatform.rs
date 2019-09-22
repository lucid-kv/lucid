#[cfg(target_os = "windows")]
fn get_binary() -> &'static str {
    "lucid.exe"
}

#[cfg(not(target_os = "windows"))]
fn get_binary() -> &'static str {
    "./lucid"
}