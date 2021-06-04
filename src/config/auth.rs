use argon2::{self, Config as Argon2Config, ThreadMode, Variant, Version};

#[derive(Debug, serde::Deserialize, Default, Clone)]
pub struct Auth {
    pub secret_key: String,
    #[serde(default)]
    pub secret_ad: String,
    pub hash_salt: String,
    #[serde(default = "exp_default")]
    pub token_expiration: u64,
}

impl Auth {
    fn get_secret_and_ad(&self) -> (&[u8], &[u8]) {
        let secret = self.secret_key.as_bytes();
        let ad = self.secret_ad.as_bytes();
        (secret, ad)
    }

    #[allow(dead_code)]
    pub fn hash_password(&self, password: &String) -> String {
        let (secret, ad) = self.get_secret_and_ad();

        let config = Argon2Config {
            variant: Variant::Argon2i,
            version: Version::Version13,
            thread_mode: ThreadMode::Parallel,
            secret,
            ad,
            hash_length: 32,
            ..Default::default()
        };

        argon2::hash_encoded(
            password.as_bytes(),
            self.hash_salt.as_bytes(),
            &config,
        )
        .unwrap()
    }

    pub fn verify_password(&self, hash: &str, password: &[u8]) -> bool {
        let (secret, ad) = self.get_secret_and_ad();

        argon2::verify_encoded_ext(hash, password, secret, ad).unwrap()
    }

    pub fn get_token_exp(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + self.token_expiration
    }
}

fn exp_default() -> u64 {
    (1 * 24 * 60 * 60) as u64 // one day in seconds
}
