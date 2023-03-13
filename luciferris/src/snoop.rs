use core::marker::Send;
use env_logger::Builder;
use log::{error, info, LevelFilter};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use sysinfo::{NetworkExt, ProcessExt, System, SystemExt};

#[derive(Default)]
struct LogDest {}

impl Write for LogDest {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}
unsafe impl Send for LogDest {}

fn out(saveto: Option<File>, buf: String) -> () {
    match saveto {
        None => info!("{}", buf),
        Some(mut file) => match file.write(buf.as_bytes()) {
            Ok(bytes) => info!("wrote {} bytes", bytes),
            Err(e) => error!("{:?}", e),
        },
    }
}

// Log everything relevant about the system
pub fn snoop(saveto: Option<&str>) -> () {
    // builder.target(Target::Pipe(Box::new(LogDest::default())));

    // Please note that we use "new_all" to ensure that all list of
    // components, network interfaces, disks and users are already
    // filled!
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    let mut out: String = String::from("");

    // We display all disks' information:
    out.insert_str(0, &format!("=> disks:"));
    for disk in sys.disks() {
        out.insert_str(0, &format!("{:?}", disk));
    }

    // Network interfaces name, data received and data transmitted:
    out.insert_str(0, &format!("=> networks:"));
    for (interface_name, data) in sys.networks() {
        out.insert_str(
            0,
            &format!(
                "{}: {}/{} B",
                interface_name,
                data.received(),
                data.transmitted()
            ),
        );
    }

    // Components temperature:
    out.insert_str(0, &format!("=> components:"));
    for component in sys.components() {
        out.insert_str(0, &format!("{:?}", component));
    }

    out.insert_str(0, &format!("=> system:"));
    // RAM and swap information:
    out.insert_str(0, &format!("total memory: {} bytes", sys.total_memory()));
    out.insert_str(0, &format!("used memory : {} bytes", sys.used_memory()));
    out.insert_str(0, &format!("total swap  : {} bytes", sys.total_swap()));
    out.insert_str(0, &format!("used swap   : {} bytes", sys.used_swap()));

    // Display system information:
    out.insert_str(0, &format!("System name:             {:?}", sys.name()));
    out.insert_str(
        0,
        &format!("System kernel version:   {:?}", sys.kernel_version()),
    );
    out.insert_str(
        0,
        &format!("System OS version:       {:?}", sys.os_version()),
    );
    out.insert_str(
        0,
        &format!("System host name:        {:?}", sys.host_name()),
    );

    // Number of CPUs:
    out.insert_str(0, &format!("NB CPUs: {}", sys.cpus().len()));

    // Display processes ID, name na disk usage:
    for (pid, process) in sys.processes() {
        out.insert_str(
            0,
            &format!("[{}] {} {:?}", pid, process.name(), process.disk_usage()),
        );
    }

    // create/open file to write to
    let mut file = match saveto {
        Some(pathbuf) => match File::create(pathbuf) {
            Ok(file) => Some(file),
            Err(e) => {
                error!("{:?}", e);
                panic!()
            }
        },
        _ => None,
    };

    match file {
        Some(mut file) => match file.write(out.as_bytes()) {
            Ok(bytes) => info!("wrote {} bytes", bytes),
            Err(e) => error!("{}", e),
        },
        None => info!("{}", out),
    }
}
