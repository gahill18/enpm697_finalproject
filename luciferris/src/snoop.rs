use env_logger::{Builder, Target};
use log::{error, info, Level, LevelFilter};
use std::fs::File;
use sysinfo::{NetworkExt, ProcessExt, System, SystemExt};

// Log everything relevant about the system
pub fn snoop() -> () {
    // Please note that we use "new_all" to ensure that all list of
    // components, network interfaces, disks and users are already
    // filled!
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    // We display all disks' information:
    info!("=> disks:{}", '\n');
    for disk in sys.disks() {
        info!("{:?}", disk);
    }

    // Network interfaces name, data received and data transmitted:
    info!("=> networks:{}", '\n');
    for (interface_name, data) in sys.networks() {
        info!(
            "{}: {}/{} B",
            interface_name,
            data.received(),
            data.transmitted()
        );
    }

    // Components temperature:
    info!("=> components:{}", '\n');
    for component in sys.components() {
        info!("{:?}", component);
    }

    info!("=> system:{}", '\n');
    // RAM and swap information:
    info!("total memory: {} bytes{}", sys.total_memory(), '\n');
    info!("used memory : {} bytes{}", sys.used_memory(), '\n');
    info!("total swap  : {} bytes{}", sys.total_swap(), '\n');
    info!("used swap   : {} bytes{}", sys.used_swap(), '\n');

    // Display system information:
    info!("System name:             {:?}{}", sys.name(), '\n');
    info!(
        "System kernel version:   {:?}{}",
        sys.kernel_version(),
        '\n'
    );
    info!("System OS version:       {:?}{}", sys.os_version(), '\n');
    info!("System host name:        {:?}{}", sys.host_name(), '\n');

    // Number of CPUs:
    info!("NB CPUs: {}", sys.cpus().len());

    // Display processes ID, name na disk usage:
    for (pid, process) in sys.processes() {
        info!("[{}] {} {:?}", pid, process.name(), process.disk_usage());
    }
}
