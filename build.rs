fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        let icon_path = std::env::var("ICON_PATH").unwrap_or_else(|_| "res/bear.ico".to_string());
        res.set_icon(&icon_path);
        res.compile().unwrap();
    }
}