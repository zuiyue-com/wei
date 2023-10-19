#![windows_subsystem = "windows"]

// include_bytes 设置的目录需要往上两层
static DATA_1: &'static [u8] = include_bytes!("../../wei-test/r");

use std::os::windows::process::CommandExt;

#[macro_use]
extern crate wei_log;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
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

    #[cfg(target_os = "windows")]{
        info!("start wei-server");
        tokio::spawn( async {
            wei_server::start().await.unwrap();
        });

        info!("start wei-ui");
        wei_ui::start().await?;
    }

    #[cfg(target_os = "windows")]{
        wei_server::start().await?;
    }

    println!("{:?}", DATA_1);

    Ok(())
}