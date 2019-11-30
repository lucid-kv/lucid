#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    if std::env::var("PROFILE").unwrap() == "release" {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/favicon.ico");
        res.compile()
            .expect("Unable to setup windows resources informations.");
    }
}

#[cfg(not(target_os = "windows"))]
fn main() {}
