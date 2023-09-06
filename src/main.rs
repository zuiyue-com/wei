#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    wei_env::bin_init("wei");

    let instance = single_instance::SingleInstance::new("wei")?;
    if !instance.is_single() { 
        std::process::exit(1);
    };

    wei_env::start();

    // 如果./data/checksum.dat不存在 
    if !std::path::Path::new("./data/checksum.dat").exists() {
        download_all().await?;
    }
    let dir = std::path::PathBuf::from("./");
    let checksums = read_checksums("./data/checksum.dat")?;
    verify_files(&checksums, &dir).await?;

    // 设置工作目录为当前 ./data
    std::env::set_current_dir("./data")?;
    wei_daemon::start().await.unwrap();

    Ok(())
}

use std::fs::{self, File};
use std::io::{self, Read, BufReader, BufRead};
use std::path::{Path};
use sha2::{Sha256, Digest};
use std::collections::HashMap;

/// 先检查 data/checksum.dat,如果不存在直接下载https://download.zuiyue.com/os/latest/data/checksum.dat，通过 checksum.dat 下载所有最新的文件和应用程序。
/// 碰到 data/checksum.dat 文件不统一的情况，先从 data/new/0.1.2 版本里面复制对应的文件到 data/ 目录下面，然后再检查，如果还是不统一，则从远程对应系统里面的latest下载所有最新的文件和应用程序。
async fn download_all() -> Result<(), Box<dyn std::error::Error>> {
    // 下载 format!("https://download.zuiyue.com/{}/latest/data/checksum.dat, std::env::consts::OS");
    // 读取checksum.dat里面的内容，内容格式是每一个文件一行，格式是 文件路径|||checksum
    // 使用 reqwest 下载这些文件对应的存放到指对应每一行的文件路径
    // Download the checksum file
    let checksum_path = Path::new("./data/checksum.dat");
    download_file("data/checksum.dat", checksum_path).await?;
    let checksums = read_checksums("./data/checksum.dat")?;
    
    for (path, _checksum) in &checksums {
        let file_path = Path::new(path);
        download_file(&path, file_path).await?;
    }

    Ok(())
}

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
                println!("Checksum mismatch for {}: expected {}, found {}", relative_path_str, expected_checksum, actual_checksum);
                copy_file_from_new_or_internet(relative_path_str).await?;
            }
        } else {
            println!("File {} not found in local directory", relative_path_str);
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
        println!("File copied successfully.");
    } else {
        println!("Source file does not exist, download from internet.");
        let path = Path::new(dest);
        download_file(dest, path).await?;
    }

    Ok(())
}

async fn download_file(file_path: &str, path: &Path) -> io::Result<()> {
    let url = format!("http://download.zuiyue.com/{}/latest/{}", std::env::consts::OS, file_path);
    println!("Downloading {} to {}", url, path.display());
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
