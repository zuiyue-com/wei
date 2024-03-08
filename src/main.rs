#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
use std::os::windows::process::CommandExt;

#[macro_use]
extern crate wei_log;

#[tokio::main(flavor = "multi_thread", worker_threads = 100)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match wei::init() {
        Ok(_) => {
            info!("init success");
        }
        Err(err) => {
            info!("init error: {}", err);
            println!("init error: {}", err);
            #[cfg(target_os = "windows")] {
                use tauri_winrt_notification::{Duration, Sound, Toast};
                Toast::new(Toast::POWERSHELL_APP_ID)
                .title("Wei")
                .text1(&err.to_string())
                .sound(Some(Sound::SMS))
                .duration(Duration::Short).show()?;
            }
        }
    };

    wei_windows::init();
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
    // 获取exe路径
    let exe_path = std::env::current_exe()?;
    // 设置exe路径为当前路径
    std::env::set_current_dir(exe_path.parent().unwrap())?;
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

    #[cfg(target_os = "windows")]
    wei_run::run("wei-ui", vec![])?;

    #[cfg(not(target_os = "windows"))]
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1000)).await;
    }

    Ok(())
}

// async fn ui() -> Result<(), Box<dyn std::error::Error>> {
//     let info = os_info::get();
//     if info.os_type() == os_info::Type::Windows {
//         let version_str = info.version().to_string();
//         let parts: Vec<&str> = version_str.split('.').collect();
//         let version = parts[0].parse::<u32>().unwrap();

//         tokio::spawn( async {
//             wei_server::start().await.unwrap();
//         });

//         if version >= 10 { //"Windows 7 以上版本"
//             wei_run::run("wei-ui", vec![])?;
//         } else {
//             match webbrowser::open("http://127.0.0.1:1115") {
//                 Ok(_) => {}
//                 Err(err) => {
//                     info!("打开网页失败,原因：{}", err);
//                 }
//             }
//             wei_tray::start().unwrap();
//         }
//     }

//     Ok(())
// }