use core::marker::Send;
use env_logger::{fmt::Target, Builder};
use log::{debug, error, info, log_enabled, LevelFilter};
use std::io::Write;
use std::{collections::HashMap, path::PathBuf};
use sysinfo::{
    Component, Disk, NetworkData, NetworkExt, Networks, NetworksExt, Pid, Process, ProcessExt,
    System, SystemExt,
};

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

fn out(saveto: &Option<PathBuf>, buf: String) -> () {
    if let Some(path) = saveto {
        info!("writing to {:?}", path);
        todo!()
    } else {
        info!("{}", buf)
    }
}

// Log everything relevant about the system
pub fn snoop(saveto: Option<PathBuf>) -> () {
    let mut builder = Builder::from_default_env();

    builder
        .format(|buf, record| -> Result<(), std::io::Error> {
            writeln!(buf, "{} - {}", record.level(), record.args())
        })
        .filter(None, LevelFilter::Info)
        .init();

    // builder.target(Target::Pipe(Box::new(LogDest::default())));

    // Please note that we use "new_all" to ensure that all list of
    // components, network interfaces, disks and users are already
    // filled!
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    // We display all disks' information:
    out(&saveto, format!("=> disks:"));
    for disk in sys.disks() {
        out(&saveto, format!("{:?}", disk));
    }

    // Network interfaces name, data received and data transmitted:
    out(&saveto, format!("=> networks:"));
    for (interface_name, data) in sys.networks() {
        out(
            &saveto,
            format!(
                "{}: {}/{} B",
                interface_name,
                data.received(),
                data.transmitted()
            ),
        );
    }

    // Components temperature:
    out(&saveto, format!("=> components:"));
    for component in sys.components() {
        out(&saveto, format!("{:?}", component));
    }

    out(&saveto, format!("=> system:"));
    // RAM and swap information:
    out(
        &saveto,
        format!("total memory: {} bytes", sys.total_memory()),
    );
    out(
        &saveto,
        format!("used memory : {} bytes", sys.used_memory()),
    );
    out(&saveto, format!("total swap  : {} bytes", sys.total_swap()));
    out(&saveto, format!("used swap   : {} bytes", sys.used_swap()));

    // Display system information:
    out(
        &saveto,
        format!("System name:             {:?}", sys.name()),
    );
    out(
        &saveto,
        format!("System kernel version:   {:?}", sys.kernel_version()),
    );
    out(
        &saveto,
        format!("System OS version:       {:?}", sys.os_version()),
    );
    out(
        &saveto,
        format!("System host name:        {:?}", sys.host_name()),
    );

    // Number of CPUs:
    out(&saveto, format!("NB CPUs: {}", sys.cpus().len()));

    // Display processes ID, name na disk usage:
    for (pid, process) in sys.processes() {
        out(
            &saveto,
            format!("[{}] {} {:?}", pid, process.name(), process.disk_usage()),
        );
    }
}
