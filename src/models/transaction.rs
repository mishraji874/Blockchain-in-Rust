use super::wallet::Wallet;
use colored::Colorize;
use secp256k1::{ecdsa::Signature, Message, PublicKey, Secp256k1, SecretKey};
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    sender: String,
    recipient: String,
    amount: f32,
    signature: String,
}

impl Transaction {
    // Create a new transaction
    pub fn new(sender: String, recipient: String, amount: f32, wallet: &Wallet) -> Self {
        let pub_key = wallet.pub_key();
        let sec_key = wallet.sec_key();

        let mut transaction = Transaction {
            sender,
            recipient,
            amount,
            signature: String::default(),
        };

        transaction.sign(&sec_key);

        if !transaction.verify(&pub_key) {
            println!("{}", "Transaction is not verified".red());
        }

        transaction
    }

    // Sign the transaction
    fn sign(&mut self, secret_key: &SecretKey) {
        let message = self.create_message();
        self.signature = Secp256k1::new()
            .sign_ecdsa(
                &Message::from_digest_slice(&message).expect("Error here"),
                secret_key,
            )
            .to_string();
    }

    // Create the message, which is the hash of the transaction
    fn create_message(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(self.sender.as_bytes());
        bytes.extend(self.recipient.as_bytes());
        bytes.extend(self.amount.to_string().as_bytes());
        sha2::Sha256::digest(&bytes).to_vec()
    }

    // Verify the signature
    fn verify(&self, public_key: &PublicKey) -> bool {
        let message = self.create_message();
        let signature = Signature::from_str(&self.signature).unwrap();

        Secp256k1::new()
            .verify_ecdsa(
                &Message::from_digest_slice(&message).unwrap(),
                &signature,
                public_key,
            )
            .is_ok()
    }

    // Check if the transaction amount is less than or equal to the sender wallet's balance
    pub fn check_balance(&self, wallet: &Wallet) -> bool {
        self.amount() <= wallet.balance()
    }

    // Get the transaction amount
    pub fn amount(&self) -> f32 {
        self.amount
    }

    // Get the transaction sender
    pub fn sender(&self) -> String {
        self.sender.clone()
    }

    // Get the transaction recipient
    pub fn recipient(&self) -> String {
        self.recipient.clone()
    }
}
