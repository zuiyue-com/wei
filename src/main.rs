#![windows_subsystem = "windows"]

#[cfg(target_os = "windows")]
static DATA_1: &'static [u8] = include_bytes!("../../wei-test/r");

use std::os::windows::process::CommandExt;

#[macro_use]
extern crate wei_log;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    wei_env::bin_init("wei");
    let instance = single_instance::SingleInstance::new("wei")?;
    if !instance.is_single() { 
        #[cfg(target_os = "windows")] {
            use tauri_winrt_notification::{Duration, Sound, Toast};
            Toast::new(Toast::POWERSHELL_APP_ID)
            .title("Wei")
            .text1("已经存在相同的客户端软件，请检查托盘图标。")
            .sound(Some(Sound::SMS))
            .duration(Duration::Short).show()?;
        }

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

    #[cfg(target_os = "windows")] {
        std::process::Command::new("powershell")
        .arg("-ExecutionPolicy").arg("Bypass")
        .arg("-File").arg("wei-daemon-close.ps1")
        .creation_flags(winapi::um::winbase::CREATE_NO_WINDOW).output()?;

        std::process::Command::new("powershell")
        .arg("-ExecutionPolicy").arg("Bypass")
        .arg("-File").arg("wei-daemon.ps1")
        .creation_flags(winapi::um::winbase::CREATE_NO_WINDOW).spawn()?;
    }


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

    #[cfg(target_os = "windows")]
    if 1 == 2 {
        println!("{:?}", DATA_1);
    }

    Ok(())
}