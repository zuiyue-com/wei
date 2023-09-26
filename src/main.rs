#![windows_subsystem = "windows"]

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
    // 配置重置状态为1
    wei_env::start();

    // 如果./data/checksum.dat不存在 
    if !std::path::Path::new("./data/checksum.dat").exists() {
        #[cfg(target_os = "windows")]
        message("错误", "文件丢失，请重新下载完整软件");

        info!("文件丢失，请重新下载完整软件")
        // download_all().await?;
    }
    let dir = std::path::PathBuf::from("./");
    let checksums = read_checksums("./data/checksum.dat")?;
    verify_files(&checksums, &dir).await?;

    // 设置工作目录为当前 ./data
    info!("set_current_dir ./data");
    std::env::set_current_dir("./data")?;

    info!("start wei await");
    wei_daemon::start().await?;

    // 退出 wei-tray 和 wei-ui
    info!("kill wei-tray and wei-ui");
    wei_run::kill("wei-tray")?;
    wei_run::kill("wei-ui")?;

    info!("exit wei");
    Ok(())
}

use std::fs::{self, File};
use std::io::{self, Read, BufReader, BufRead};
use std::path::{Path};
use sha2::{Sha256, Digest};
use std::collections::HashMap;

fn calculate_sha256<P: AsRef<Path>>(file_path: P) -> io::Result<String> {
    let mut file = File::open(file_path.as_ref())?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut hasher = Sha256::new();
    hasher.update(buffer);
    let hash = hasher.finalize();
    Ok(format!("{:x}", hash))
}

fn read_checksums<P: AsRef<Path>>(file_path: P) -> io::Result<HashMap<String, String>> {
    let file = File::open(file_path)?;

    let reader = BufReader::new(file);

    let mut checksums = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split("|||");
        let path = parts.next().unwrap().to_string();
        let checksum = parts.next().unwrap().to_string();
        checksums.insert(path, checksum);
    }

    Ok(checksums)
}
async fn verify_files(checksums: &HashMap<String, String>, prefix: &Path) -> io::Result<()> {
    for relative_path_str in checksums.keys() {
        let path = prefix.join(relative_path_str);

        if path.file_name().unwrap() == "checksum.dat" ||
           path.file_name().unwrap() == "wei.exe" {
            continue;
        }

        if path.is_file() {
            let expected_checksum = checksums.get(relative_path_str).unwrap();
            let actual_checksum = calculate_sha256(&path)?;
            if &actual_checksum != expected_checksum {
                info!("Checksum mismatch for {}: expected {}, found {}", relative_path_str, expected_checksum, actual_checksum);
                copy_file_from_new_or_internet(relative_path_str).await?;
            }
        } else {
            info!("File {} not found in local directory", relative_path_str);
            copy_file_from_new_or_internet(relative_path_str).await?;
        }
    }

    Ok(())
}

async fn copy_file_from_new_or_internet(dest: &str) -> std::io::Result<()> {
    let local_version = fs::read_to_string("./data/version.dat").unwrap();
    let src = format!("./data/new/{}/{}", local_version, dest);
    if Path::new(&src).exists() {
        fs::copy(src, dest)?;
        info!("File copied successfully.");
    } else {
        info!("Source file does not exist.");
        #[cfg(target_os = "windows")]
        message("提示", "文件丢失或者不匹配，请重新下载最新版本");
        info!("文件丢失或者不匹配，请重新下载最新版本");

        std::process::exit(1);
        // let path = Path::new(dest);
        // download_file(dest, path).await?;
    }

    Ok(())
}

#[cfg(target_os = "windows")]
fn message(title: &str, text: &str) {
    use winapi::um::winuser::{MessageBoxW, MB_OK};
    use std::os::windows::ffi::OsStrExt;
    use std::ffi::OsStr;
    use std::iter::once;
    use std::ptr::null_mut;

    let title: Vec<u16> = OsStr::new(title).encode_wide().chain(once(0)).collect();
    let text: Vec<u16> = OsStr::new(text).encode_wide().chain(once(0)).collect();
    
    unsafe {
        MessageBoxW(null_mut(), text.as_ptr(), title.as_ptr(), MB_OK);
    }
}


// 先检查 data/checksum.dat,如果不存在直接下载https://download.zuiyue.com/os/latest/data/checksum.dat，通过 checksum.dat 下载所有最新的文件和应用程序。
// 碰到 data/checksum.dat 文件不统一的情况，先从 data/new/0.1.2 版本里面复制对应的文件到 data/ 目录下面，然后再检查，如果还是不统一，则从远程对应系统里面的latest下载所有最新的文件和应用程序。
async fn _download_all() -> Result<(), Box<dyn std::error::Error>> {
    // 下载 format!("https://download.zuiyue.com/{}/latest/data/checksum.dat, std::env::consts::OS");
    // 读取checksum.dat里面的内容，内容格式是每一个文件一行，格式是 文件路径|||checksum
    // 使用 reqwest 下载这些文件对应的存放到指对应每一行的文件路径
    // Download the checksum file
    let checksum_path = Path::new("./data/checksum.dat");
    _download_file("data/checksum.dat", checksum_path).await?;
    let checksums = read_checksums("./data/checksum.dat")?;
    
    for (path, _checksum) in &checksums {
        let file_path = Path::new(path);
        _download_file(&path, file_path).await?;
    }

    Ok(())
}


async fn _download_file(file_path: &str, path: &Path) -> io::Result<()> {
    let url = format!("http://download.zuiyue.com/{}/latest/{}", std::env::consts::OS, file_path);
    if path.display().to_string() == "wei.exe" {
        return Ok(());
    }
    info!("Downloading {} to {}", url, path.display());
    // Create parent directory if it doesn't exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let response = reqwest::get(url).await.unwrap();

    let content = response.bytes().await.unwrap();
    // Write the file to disk
    fs::write(path, content)?;

    Ok(())
}