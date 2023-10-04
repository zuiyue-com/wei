#![windows_subsystem = "windows"]

#[macro_use]
extern crate wei_log;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    wei_run::run("wei-daemon", vec![])?;

    #[cfg(target_os = "windows")]    
    std::process::Command::new("powershell")
        .arg("-ExecutionPolicy").arg("Bypass")
        .arg("-File").arg("wei-daemon.ps1")
        .creation_flags(winapi::um::winbase::CREATE_NO_WINDOW).output()?;
    
    info!("kill wei-tray and wei-ui");
    wei_run::kill("wei-tray")?;
    wei_run::kill("wei-ui")?;

    info!("exit wei");
    Ok(())
}