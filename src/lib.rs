#[macro_use]
extern crate wei_log;

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    // 读取first.dat文件，如果存在则退出
    let home_dir = wei_env::home_dir()?;
    let first_dat = home_dir + "first.dat";
    
    let data = match std::fs::read_to_string(&first_dat) {
        Ok(data) => data,
        Err(_) => "".to_string(),
    };

    info!("data: {}", data);

    if data != "".to_string() {
        return Ok(());
    }

    let info = os_info::get();
    if info.os_type() == os_info::Type::Windows {
        let version_str = info.version().to_string();
        let parts: Vec<&str> = version_str.split('.').collect();
        let version = parts[0].parse::<u32>()?;

        if version < 10 { //"Windows 10 以下版本"
            install()?;
        }
    }

    std::fs::write(&first_dat, "1")?;
    
    Ok(())
    // 检查是不是windows 7系统
    // 下载安装 https://msedge.sf.dl.delivery.mp.microsoft.com/filestreamingservice/files/cc99bab3-b3ec-452d-a57e-2aa38399ccaa/MicrosoftEdgeWebview2Setup.exe
    // 安装完毕后重启电脑
    // 重新运行wei-ui
}


fn install() -> Result<(), Box<dyn std::error::Error>> {
    toast("正在安装Windows6.1-KB3080149-x64.msu，请稍候。")?;
    let url = "https://download.microsoft.com/download/4/E/8/4E864B31-7756-4639-8716-0379F6435016/Windows6.1-KB3080149-x64.msu";
    let dest = std::env::current_dir()?;
    let dest = dest.join("data").join("1.msu");
    let expected_size = 17678484; // 预期大小，需要替换为实际值
    let data = download_file_and_verify_size(url, dest.to_str().unwrap(), expected_size)?;
    
    if !data {
        toast("下载Windows6.1-KB3080149-x64.msu，请稍候再试。")?;
        return Err("download_file_and_verify_size error".into());
    }

    // 获取当前路径
    info!("dest: {:?}", dest.to_str().unwrap());
    let data = wei_run::command(dest.to_str().unwrap(), vec![])?;
    if data.contains("error") || data.contains("错误") || data.contains("失败") {
        toast("安装Windows6.1-KB3080149-x64.msu，请稍候再试。")?;
        return Err("安装Windows6.1-KB3080149-x64.msu，请稍候再试。".into());
    }

    toast("正在安装Microsoft Edge WebView2 Runtime，请稍候。")?;
    let url = "https://msedge.sf.dl.delivery.mp.microsoft.com/filestreamingservice/files/94423d2e-c211-4c2d-b997-29baedb00650/MicrosoftEdgeWebView2RuntimeInstallerX64.exe";
    let dest = std::env::current_dir()?;
    let dest = dest.join("data").join("2.exe");
    let expected_size = 177177032; // 预期大小，需要替换为实际值
    let data = download_file_and_verify_size(url, dest.to_str().unwrap(), expected_size)?;

    if !data {
        toast("下载Microsoft Edge WebView2 Runtime失败，请稍候再试。")?;
        return Err("download_file_and_verify_size error".into());
    }

    info!("dest: {:?}", dest.to_str().unwrap());
    let data = wei_run::command(dest.to_str().unwrap(), vec![])?;
    if data.contains("error") || data.contains("错误") || data.contains("失败") {
        toast("安装Microsoft Edge WebView2 Runtime失败，请稍候再试。")?;
        return Err("安装Microsoft Edge WebView2 Runtime失败，请稍候再试。".into());
    }
    

    Ok(())
}

fn toast(data: &str) -> Result<(), Box<dyn std::error::Error>> {
    use tauri_winrt_notification::{Duration, Sound, Toast};
    Toast::new(Toast::POWERSHELL_APP_ID)
    .title("Wei")
    .text1(data)
    .sound(Some(Sound::SMS))
    .duration(Duration::Short).show()?;

    Ok(())
}

use std::io::Write;
use std::fs::File;

fn download_file_and_verify_size(url: &str, dest: &str, expected_size: u64) -> Result<bool, Box<dyn std::error::Error>> {
    let mut response = ureq::get(url).call()?.into_reader();
    let mut out = File::create(dest)?;
    
    let mut buffer = [0;1024];
    let mut total_size = 0;
    while let Ok(n) = response.read(&mut buffer) {
        if n == 0 {
            break;
        }
        out.write_all(&buffer[..n])?;
        total_size += n as u64;
    }

    if total_size != expected_size {
        info!("Downloaded data size is incorrect");
        return Ok(false);
    }

    Ok(true)
}



// pub fn elevated(path: &str) -> bool {
//     use winapi::um::shellapi::{ShellExecuteW};
//     use winapi::um::winuser::{SW_SHOW};
//     use std::os::windows::prelude::*;
//     use std::ptr::null_mut;
//     use std::ffi::OsStr;
//     use std::iter::once;

//     let args: Vec<_> = std::env::args().skip(1).collect();
//     let args = args.join(" ");
    
//     let exe_path_str = OsStr::new(path)
//         .encode_wide()
//         .chain(once(0))
//         .collect::<Vec<u16>>();
//     let operation = OsStr::new("runas")
//         .encode_wide()
//         .chain(once(0))
//         .collect::<Vec<u16>>();
//     let args = OsStr::new(&args)
//         .encode_wide()
//         .chain(once(0))
//         .collect::<Vec<u16>>();

//     let result = unsafe {
//         ShellExecuteW(
//             null_mut(),
//             operation.as_ptr(),
//             exe_path_str.as_ptr(),
//             args.as_ptr(),
//             null_mut(),
//             SW_SHOW,
//         )
//     };

//     if (result as isize) <= 32 {
//         return false;
//     }
//     true
// }