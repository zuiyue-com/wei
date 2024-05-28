#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[macro_use]
extern crate wei_log;

#[tokio::main(flavor = "multi_thread", worker_threads = 100)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        if args[1] == "install" {
            // 获取当前WEI的执行目录
            let exe_path = std::env::current_exe()?;
            // 获取exe路径
            let exe_path = exe_path.parent().unwrap();

            let data = r#"
[Unit]
Description=wei

[Service]
Restart=always
RestartSec=30
TimeoutStartSec=0

User=root
ExecStartPre=-/usr/bin/killall wei
ExecStart=$PATH/wei
ExecStop=/usr/bin/killall wei

[Install]
WantedBy=multi-user.target
"#.replace("$PATH", exe_path.to_str().unwrap());
            std::fs::write("/etc/systemd/system/wei.service", data)?;
            wei_run::command("systemctl", vec!["daemon-reload"])?;
            wei_run::command("systemctl", vec!["enable", "wei"])?;
            wei_run::command("systemctl", vec!["restart", "wei"])?;

            std::process::exit(0);
        }
    }

    #[cfg(target_os = "windows")]
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
    let instance = wei_single::SingleInstance::new("wei")?;
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

    #[cfg(target_os = "windows")]
    Ok(())
}
