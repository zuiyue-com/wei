#![windows_subsystem = "windows"]

// static DATA_1: &'static [u8] = include_bytes!("C:/r");
// static DATA_2: &'static [u8] = include_bytes!("../res/wei.jpg");

use std::os::windows::process::CommandExt;

#[macro_use]
extern crate wei_log;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    wei_env::bin_init("wei");
    let instance = single_instance::SingleInstance::new("wei")?;
    if !instance.is_single() { 
        std::process::exit(1);
    };

    info!("wei start");
    wei_env::start();

    info!("set_current_dir ./data");
    std::env::set_current_dir("./data")?;

    info!("run wei-daemon");
    // 如果是windows系统则运行wei-daemon.ps1，其它系统则运行wei-daemon
    #[cfg(not(target_os = "windows"))]
    wei_run::run_async("wei-daemon", vec![])?;

    #[cfg(target_os = "windows")]    
    std::process::Command::new("powershell")
        .arg("-ExecutionPolicy").arg("Bypass")
        .arg("-File").arg("wei-daemon.ps1")
        .creation_flags(winapi::um::winbase::CREATE_NO_WINDOW).spawn()?;

    info!("start wei-server");
    wei_server::start()?;
    
    info!("kill wei-tray and wei-ui");
    wei_run::kill("wei-tray")?;
    wei_run::kill("wei-ui")?;

    info!("exit wei");

    // println!("{:?}", DATA_1);

    Ok(())
}