use std::path::PathBuf;

pub fn expand_user_path(path: &str) -> PathBuf {
    if let Some(stripped) = path.strip_prefix("~/") {
        if let Some(home_dir) = home::home_dir() {
            return home_dir.join(stripped);
        }
    }
    PathBuf::from(path)
}
