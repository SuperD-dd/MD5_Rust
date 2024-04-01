use std::fs::File;
use std::io::{self, Read};
use serde::{Deserialize, Serialize};
//use md5::Context as MD5Context;
use sha1::{Sha1, Digest};
use crc32fast::Hasher as CRC32Hasher;
use base64ct::{Base64, Encoding};
use std::time::SystemTime;
use chrono::{DateTime, Local, NaiveDateTime};
use super::error::MD5Error;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SupportDeliverie {
    pub name: String,
}
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
// #[tauri::command(async)]
// pub async fn greet(name: &str) -> Result<SupportDeliverie, MD5Error> {
//     Ok(SupportDeliverie{
//         name :format!("Hello, {}! You've been greeted from Rust!", name),
//     })
// }

// #[tauri::command]
// fn calculate_md5(file_path: &str) -> io::Result<String> {
//     let mut file = File::open(file_path)?;
//     let mut md5_context = MD5Context::new();

//     let mut buffer = [0; 4096];
//     loop {
//         let bytes_read = file.read(&mut buffer)?;
//         if bytes_read == 0 {
//             break;
//         }
//         md5_context.consume(&buffer[..bytes_read]);
//     }

//     let result = format!("{:x}", md5_context.compute());
//     Ok(format!("{}", result).to_uppercase())
// }

#[tauri::command]
fn calculate_sha1(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut sha1_context = Sha1::new();

    let mut buffer = [0; 4096];
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        sha1_context.update(&buffer[..bytes_read]);
    }

    let result = format!("{:x}", sha1_context.finalize());
    
    Ok(result.to_uppercase())
}

#[tauri::command]
fn get_file_info(file_path: &str) -> io::Result<( u64, String)> {
    // 获取文件元数据
    let metadata = std::fs::metadata(file_path)?;

    // 获取文件大小
    let file_size = metadata.len();

    // 获取最近修改时间
    let modified_time:SystemTime = metadata.modified()?;

    let local_time: DateTime<Local> = modified_time.into();
    Ok((file_size, local_time.format("%Y年%m月%d日,%H:%M:%S").to_string()))
}