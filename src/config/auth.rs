use argon2::{self, Config as Argon2Config, ThreadMode, Variant, Version};

#[derive(Debug, serde::Deserialize, Default, Clone)]
pub struct Auth {
    pub secret_key: String,
    #[serde(default)]
    pub secret_ad: String,
    pub hash_salt: String,
}

impl Auth {
    fn get_secret_and_ad(&self) -> (&[u8], &[u8]) {
        let secret = self.secret_key.as_bytes();
        let ad = self.secret_ad.as_bytes();
        (secret, ad)
    }

    fn get_config(&self) -> Argon2Config {
        let (secret, ad) = self.get_secret_and_ad();

        Argon2Config {
            variant: Variant::Argon2i,
            version: Version::Version13,
            thread_mode: ThreadMode::Parallel,
            secret,
            ad,
            hash_length: 32,
            ..Default::default()
        }
    }

    pub fn hash_password(&self, password: &String) -> String {
        argon2::hash_encoded(
            password.as_bytes(),
            self.hash_salt.as_bytes(),
            &self.get_config(),
        )
        .unwrap()
    }

    pub fn verify_password(&self, hash: &str, password: &[u8]) -> bool {
        let (secret, ad) = self.get_secret_and_ad();

        argon2::verify_encoded_ext(hash, password, secret, ad).unwrap()
    }
}
