use std::{
    fs::OpenOptions,
    io::{BufReader, BufWriter},
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::{Ok, Result};
use secp256k1::{rand::rngs, PublicKey, SecretKey};
use serde::{Deserialize, Serialize};
use tiny_keccak::keccak256;
use web3::types::Address;

#[derive(Debug, Serialize, Deserialize)]
pub struct EthWallet {
    secret_key: String,
    public_key: String,
    pub public_address: String,
}

impl EthWallet {
    pub fn new(secret_key: &SecretKey, public_key: &PublicKey) -> Self {
        let public_address = public_key_address(&public_key);
        let address = format!("{:?}", public_address);
        EthWallet {
            secret_key: secret_key.to_string(),
            public_key: public_key.to_string(),
            public_address: address,
        }
    }

    pub fn save_to_file(&self) -> Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open("wallet.json")?;
        let buf_writer = BufWriter::new(file);
        serde_json::to_writer_pretty(buf_writer, self)?;

        Ok(())
    }

    pub fn load_from_file() -> Result<Self> {
        let file = OpenOptions::new().read(true).open("wallet.json")?;
        let buf_reader = BufReader::new(file);

        let wallet: Self = serde_json::from_reader(buf_reader)?;
        Ok(wallet)
    }

    pub fn get_secret_key(&self) -> Result<SecretKey> {
        let secret_key = SecretKey::from_str(&self.secret_key)?;
        Ok(secret_key)
    }

    pub fn get_public_key(&self) -> Result<PublicKey> {
        let public_key = PublicKey::from_str(&self.public_key)?;
        Ok(public_key)
    }
}

pub fn get_time_in_nanoseconds() -> u64 {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    timestamp.as_secs() << 30 | timestamp.subsec_nanos() as u64
}

pub fn generate_keypairs() -> (SecretKey, PublicKey) {
    let secp = secp256k1::Secp256k1::new();
    let mut random_number_generator = rngs::JitterRng::new_with_timer(get_time_in_nanoseconds);
    secp.generate_keypair(&mut random_number_generator)
}

pub fn public_key_address(public_key: &PublicKey) -> Address {
    let public_key = public_key.serialize_uncompressed();

    debug_assert_eq!(public_key[0], 0x04);
    let hash = keccak256(&public_key[1..]);

    Address::from_slice(&hash[12..])
}
