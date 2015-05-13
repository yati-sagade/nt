use std::env;
use std::path::PathBuf;

pub fn nt_dir() -> Option<PathBuf> {
    match env::home_dir() {
        Some(mut dir) => {
            dir.push(".nt");
            Some(dir)
        },
        None => None,
    }
}

