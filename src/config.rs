use std::path::PathBuf;

use anyhow::Context;
use dirs::config_dir;
use rand::thread_rng;
use rsa::pkcs8::LineEnding;
use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey};
use secrecy::SecretString;

#[derive(Debug)]
pub struct Config {
    public_key: String,
    private_key: SecretString,
}

impl Config {
    pub fn new() -> Self {
        let config_dir = &config_dir()
            .expect("Failed to get config directory")
            .join("sleepy_owl");

        if !config_dir.exists() {
            std::fs::create_dir_all(config_dir).expect("Failed to create config directory");
        }

        let private_key_path = config_dir.join("private_key.pem");
        let public_key_path = config_dir.join("public_key.pem");

        if !private_key_path.exists() || !public_key_path.exists() {
            Self::generate_key(config_dir).expect("Failed to generate RSA key pair");
        }

        let private_key = std::fs::read_to_string(&private_key_path)
            .expect("Failed to read private key from file");

        let public_key =
            std::fs::read_to_string(&public_key_path).expect("Failed to read public key from file");

        Config {
            public_key,
            private_key: SecretString::new(private_key.into()),
        }
    }

    fn generate_key(config_dir: &PathBuf) -> anyhow::Result<()> {
        let private_key = rsa::RsaPrivateKey::new(&mut thread_rng(), 2048)
            .context("Failed to generate RSA key")?;

        let public_key = rsa::RsaPublicKey::from(&private_key);

        let private_key_pem = private_key.to_pkcs8_pem(LineEnding::LF)?;
        let public_key_pem = public_key.to_public_key_pem(LineEnding::LF)?;

        std::fs::write(
            config_dir.join("private_key.pem"),
            private_key_pem.as_bytes(),
        )
        .context("Failed to write private key to file")?;
        std::fs::write(config_dir.join("public_key.pem"), public_key_pem.as_bytes())
            .context("Failed to write public key to file")?;

        Ok(())
    }
}
