use secp256k1::{PublicKey, Secp256k1, SecretKey};
use serde::{Deserialize, Serialize};
use serde_json;
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{self, Read},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    name: String,
    #[serde(with = "serde_bytes_secret_key")]
    sec_key: SecretKey,
    #[serde(with = "serde_bytes_public_key")]
    pub_key: PublicKey,
    balance: f32,
}

impl Wallet {
    // Create a new wallet
    pub fn new(name: String, secret: String) -> Self {
        let sec_engine = Secp256k1::new();
        let sec_key = SecretKey::from_slice(&Sha256::digest(secret.as_bytes()))
            .expect("32 bytes, within curve order");
        let pub_key = sec_key.public_key(&sec_engine);

        let wallet = Wallet {
            name,
            sec_key,
            pub_key,
            balance: 1000.0,
        };

        // Save the wallet to a JSON file after creation
        wallet
            .save_to_file()
            .expect("Failed to save wallet to file");

        wallet
    }

    // get the secret key of the wallet
    pub fn sec_key(&self) -> &SecretKey {
        &self.sec_key
    }

    // get the public key of the wallet
    pub fn pub_key(&self) -> &PublicKey {
        &self.pub_key
    }

    // get the balance of the wallet
    pub fn balance(&self) -> f32 {
        self.balance
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    // update the balance of the wallet
    pub fn update_balance(&mut self, amount: f32) {
        self.balance -= amount; // Deduct the amount from the balance
        self.save_to_file()
            .expect("Failed to update wallet in file"); // Update the file
    }

    pub fn get_wallets() -> Option<HashMap<String, Wallet>> {
        let file_path = "wallets.json";

        let wallets: HashMap<String, Wallet> =
            if let Ok(mut file) = OpenOptions::new().read(true).open(file_path) {
                let mut contents = String::new();
                file.read_to_string(&mut contents)
                    .expect("Failed to read wallet file");
                serde_json::from_str(&contents).unwrap_or_else(|e| {
                    eprintln!("Failed to deserialize wallets: {}", e);
                    HashMap::new()
                })
            } else {
                // Create two new wallets
                let wallet1 = Wallet::new("Alice".to_owned(), "Alice".to_owned());
                let wallet2 = Wallet::new("Bob".to_owned(), "Bob".to_owned());
                HashMap::from([("Alice".to_owned(), wallet1), ("Bob".to_owned(), wallet2)])
            };

        Some(wallets)
    }

    // Save the wallet to a JSON file
    fn save_to_file(&self) -> io::Result<()> {
        let file_path = "wallets.json";

        // Try to open the file in read mode first to gather existing wallets
        let mut wallets: HashMap<String, Wallet> =
            if let Ok(mut file) = OpenOptions::new().read(true).open(file_path) {
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;

                // Deserialize existing wallets
                serde_json::from_str(&contents).unwrap_or_else(|e| {
                    eprintln!("Failed to deserialize wallets: {}", e);
                    HashMap::new()
                })
            } else {
                HashMap::new()
            };

        // Insert the new wallet
        wallets.insert(self.name.clone(), self.clone());

        // Write the updated hashmap back to the file in pretty format
        let file = OpenOptions::new()
            .write(true)
            .truncate(true) // This will overwrite the file with the new content
            .create(true)
            .open(file_path)?; // Open the file for writing

        serde_json::to_writer_pretty(file, &wallets)?; // Use to_writer_pretty for formatted JSON
        Ok(())
    }
}

// Serialize and deserialize the secret key as bytes
mod serde_bytes_secret_key {
    use secp256k1::SecretKey;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(key: &SecretKey, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(key.as_ref())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<SecretKey, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes: Vec<u8> = Deserialize::deserialize(deserializer)?;
        SecretKey::from_slice(bytes.as_slice()).map_err(serde::de::Error::custom)
    }
}

// Serialize and deserialize the public key as bytes
mod serde_bytes_public_key {
    use secp256k1::PublicKey;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(key: &PublicKey, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize the PublicKey directly as bytes
        serializer.serialize_bytes(&key.serialize())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<PublicKey, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes: Vec<u8> = Deserialize::deserialize(deserializer)?;
        PublicKey::from_slice(bytes.as_slice()).map_err(serde::de::Error::custom)
    }
}

// Display the wallet
impl std::fmt::Display for Wallet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Wallet: {} - Balance: {} - Secret Key: {:?} - Public Key: {:?}",
            self.name,
            self.balance,
            self.sec_key(),
            self.pub_key()
        )
    }
}
