pub struct AppConfig {
    pub root_dir: std::path::PathBuf,
}

impl AppConfig {
    /**
     * Create a new AppConfig, and sets the root dir to
     * the user's data_dir + path postfix (e.g. ~/.config/postfix)
     */
    pub fn new(path_postfix: &str) -> AppConfig {
        let mut root_dir = std::path::PathBuf::from(match dirs::home_dir() {
            Some(dir) => dir,
            None => std::path::PathBuf::default(),
        });
        root_dir.push(path_postfix);
        AppConfig { root_dir }
    }
}
