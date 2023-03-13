use core::marker::Send;
use env_logger::{Builder, Target};
use log::{error, info, LevelFilter};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use sysinfo::{NetworkExt, ProcessExt, System, SystemExt};

// Log everything relevant about the system
pub fn snoop(saveto: Option<&str>) -> () {
    let mut builder = Builder::from_default_env();
    if let Some(s) = saveto {
        match File::create(s) {
            Ok(file) => {
                builder.target(Target::Pipe(Box::new(file)));
            }
            Err(e) => error!("{:?}", e),
        }
    }
    builder.filter(None, LevelFilter::Info).init();

    // Please note that we use "new_all" to ensure that all list of
    // components, network interfaces, disks and users are already
    // filled!
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    // We display all disks' information:
    info!("=> disks:");
    for disk in sys.disks() {
        info!("{:?}", disk);
    }

    // Network interfaces name, data received and data transmitted:
    info!("=> networks:");
    for (interface_name, data) in sys.networks() {
        info!(
            "{}: {}/{} B",
            interface_name,
            data.received(),
            data.transmitted()
        );
    }

    // Components temperature:
    info!("=> components:");
    for component in sys.components() {
        info!("{:?}", component);
    }

    info!("=> system:");
    // RAM and swap information:
    info!("total memory: {} bytes", sys.total_memory());
    info!("used memory : {} bytes", sys.used_memory());
    info!("total swap  : {} bytes", sys.total_swap());
    info!("used swap   : {} bytes", sys.used_swap());

    // Display system information:
    info!("System name:             {:?}", sys.name());
    info!("System kernel version:   {:?}", sys.kernel_version());
    info!("System OS version:       {:?}", sys.os_version());
    info!("System host name:        {:?}", sys.host_name());

    // Number of CPUs:
    info!("NB CPUs: {}", sys.cpus().len());

    // Display processes ID, name na disk usage:
    for (pid, process) in sys.processes() {
        info!("[{}] {} {:?}", pid, process.name(), process.disk_usage());
    }
}
