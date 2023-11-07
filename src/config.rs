use std::{fs::create_dir, path::PathBuf};

pub fn define_directory() -> PathBuf {
    let path = match dirs::home_dir() {
        Some(p) => p,
        None => PathBuf::new(),
    };

    path.join(".dawn")
}

pub fn check_directory(path: &PathBuf) {
    if !path.exists() {
        create_dir(&path).expect("Directory creation failed");
        println!("...Directory created");
    }
}
