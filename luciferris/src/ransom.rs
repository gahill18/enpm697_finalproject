use env_logger::{fmt::Target, Builder};
use log::{debug, error, info, log_enabled, LevelFilter};
use std::fs::{File, ReadDir};

pub fn ransom() {
    println!("ransom mode")
}

fn encrypt_files(files: ReadDir) -> Result<(), &'static str> {
    for file in files {
        info!("{:?}", file)
    }
    Ok(())
}

fn encrypt(file: File) -> Result<(), &'static str> {
    Ok(())
}
