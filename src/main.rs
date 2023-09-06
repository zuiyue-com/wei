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
    verify_files(&checksums, &dir)?;

    wei_daemon::start().await.unwrap();

    Ok(())
}

use std::fs::{self, File};
use std::io::{self, Read, BufReader, BufRead};
use std::path::{Path};
use sha2::{Sha256, Digest};
use std::collections::HashMap;

async fn download_all() -> Result<(), Box<dyn std::error::Error>> {
    // 下载 format!("https://download.zuiyue.com/{}/latest/data/checksum.dat, std::env::consts::OS");
    // 读取checksum.dat里面的内容，内容格式是每一个文件一行，格式是 文件路径|||checksum
    // 使用 reqwest 下载这些文件对应的存放到指对应每一行的文件路径
    // Download the checksum file
    let checksum_url = format!("http://download.zuiyue.com/{}/latest/data/checksum.dat", std::env::consts::OS);
    println!("Downloading checksum file from {}", checksum_url);
    let checksum_path = Path::new("./data/checksum.dat");
    download_file(&checksum_url, checksum_path).await?;
    let checksums = read_checksums("./data/checksum.dat")?;
    
    for (path, _checksum) in &checksums {
        let file_url = format!("http://download.zuiyue.com/{}/latest/{}", std::env::consts::OS, path);
        let file_path = Path::new(path);
        download_file(&file_url, file_path).await?;
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
fn verify_files(checksums: &HashMap<String, String>, prefix: &Path) -> io::Result<()> {
    for relative_path_str in checksums.keys() {
        let path = prefix.join(relative_path_str);

        if path.file_name() == Some(std::ffi::OsStr::new("checksum.dat")) {
            continue;
        }

        if path.is_file() {
            let expected_checksum = checksums.get(relative_path_str).unwrap();
            let actual_checksum = calculate_sha256(&path)?;
            if &actual_checksum != expected_checksum {
                println!("Checksum mismatch for {}: expected {}, found {}", relative_path_str, expected_checksum, actual_checksum);
                // TODO: Handle mismatch
            }
        } else {
            println!("File {} not found in local directory", relative_path_str);
            // TODO: Handle missing file
        }
    }

    Ok(())
}

async fn download_file(url: &str, path: &Path) -> io::Result<()> {
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
