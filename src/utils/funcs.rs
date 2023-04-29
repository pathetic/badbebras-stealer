extern crate base64;
extern crate hex;

use core::str;
use std::{
    fs::File,
    fs::{self},
    io::prelude::*,
    io::{Seek, Write},
    iter::Iterator,
    path::{Path, PathBuf},
};
use walkdir::{DirEntry, WalkDir};
use zip::result::ZipError;
use zip::write::FileOptions;

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use rand::Rng;

const SUFFIX: [&'static str; 9] = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

pub fn convert_bytes<T: Into<f64>>(size: T) -> String {
    let size = size.into();

    if size <= 0.0 {
        return "0 B".to_string();
    }

    let base = size.log10() / 1024_f64.log10();

    let mut result = ((1024_f64.powf(base - base.floor()) * 10.0).round() / 10.0).to_string();

    result.push(' ');
    result.push_str(SUFFIX[base.floor() as usize]);

    result
}

fn zip_dir<T>(
    it: &mut dyn Iterator<Item = DirEntry>,
    prefix: &str,
    writer: T,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();
        if path.is_file() {
            #[allow(deprecated)]
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;
            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            #[allow(deprecated)]
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    Result::Ok(())
}

pub fn zip_it(src_dir: &str, dst_file: &str) -> zip::result::ZipResult<()> {
    if !Path::new(src_dir).is_dir() {
        return Err(ZipError::FileNotFound);
    }

    let path = Path::new(dst_file);
    let file = File::create(path).unwrap();

    let walkdir = WalkDir::new(src_dir);
    let it = walkdir.into_iter();

    zip_dir(
        &mut it.filter_map(|e| e.ok()),
        src_dir,
        file,
        zip::CompressionMethod::Stored,
    );

    Ok(())
}

pub fn delete_folder(path: &str) -> std::io::Result<()> {
    std::fs::remove_dir_all(path)
}

pub fn delete_file(path: &str) -> std::io::Result<()> {
    std::fs::remove_file(path)
}

pub fn try_create(path: PathBuf) {
    if !path.exists() {
        fs::create_dir(&path).unwrap();
    } else {
        fs::remove_dir_all(&path).unwrap();
        fs::create_dir(&path).unwrap();
    }
}

fn generate_random_bytes() -> Vec<u8> {
    Vec::from(rand::thread_rng().gen::<[u8; 12]>())
}

pub async fn encrypt_file(path: &Path, key: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let buffer = std::fs::read(path).unwrap();
    std::fs::remove_file(path).unwrap();
    let cipher = Aes256Gcm::new_from_slice(key).unwrap();
    let nonce = generate_random_bytes();
    let nonce = Nonce::from_slice(&nonce);
    println!("{:?}", base64::encode(nonce));

    let buffer = cipher.encrypt(nonce, buffer.as_ref()).unwrap();
    let enc_path = format!("{}.enc", std::path::Path::to_str(&path).unwrap());
    let mut file = std::fs::File::create(&enc_path).unwrap();
    file.write_all(&nonce).unwrap();
    file.write_all(&buffer).unwrap();
    print!("{} ", buffer.len());
    drop(file);
    std::fs::rename(enc_path, path);

    Ok(())
}

pub fn decrypt_file(path: &Path, key: &[u8]) {
    let buffer = std::fs::read(path).unwrap();
    let cipher = Aes256Gcm::new_from_slice(key).unwrap();
    let nonce = Nonce::from_slice(&buffer[..12]);
    println!("{:?}", nonce);
    let buffer = cipher.decrypt(nonce, &buffer[12..]).unwrap();
    std::fs::write(path, &buffer);
}

// pub fn copy_directory<U: AsRef<Path>, V: AsRef<Path>>(
//     src: U,
//     dst: V,
// ) -> Result<(), std::io::Error> {
//     let mut stack = Vec::new();
//     stack.push(PathBuf::from(src.as_ref()));
//     let output_root = PathBuf::from(dst.as_ref());
//     let input_root = PathBuf::from(src.as_ref()).components().count();
//     while let Some(working_path) = stack.pop() {
//         let src: PathBuf = working_path.components().skip(input_root).collect();
//         let dest = if src.components().count() == 0 {
//             output_root.clone()
//         } else {
//             output_root.join(&src)
//         };
//         if fs::metadata(&dest).is_err() {
//             fs::create_dir_all(&dest)?;
//         }
//         for entry in fs::read_dir(working_path)? {
//             let entry = entry?;
//             if entry.file_type()?.is_dir() {
//                 stack.push(entry.path());
//             } else {
//                 if let Some(filename) = entry.path().file_name() {
//                     fs::copy(&entry.path(), &dest.join(filename))?;
//                 }
//             }
//         }
//     }
//     Ok(())
// }
