use std::path::PathBuf;

/// Expands `~/` paths to user's home directory. Returns unchanged path if no `~/` prefix or home dir not found.
pub fn expand_user_path(path: &str) -> PathBuf {
    if let Some(stripped) = path.strip_prefix("~/") {
        if let Some(home_dir) = home::home_dir() {
            return home_dir.join(stripped);
        }
    }
    PathBuf::from(path)
}
