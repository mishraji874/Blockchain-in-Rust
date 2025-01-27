# Blockchain in Rust

This project implements a basic blockchain in Rust, demonstrating core concepts like block creation, data storage, and a simple proof-of-work consensus mechanism.

## Key Features:

* **Block Creation:**
    * Creates new blocks with a unique hash and links them to the previous block in the chain.
    * Includes a timestamp and a nonce for each block.
* **Data Storage:** 
    * Stores data within each block.
    * Allows for the addition of new data to the blockchain.
* **Proof-of-Work:** 
    * Implements a simplified proof-of-work algorithm to secure the blockchain and prevent tampering.
* **Chain Validation:** 
    * Includes basic validation checks to ensure the integrity of the blockchain.

**Getting Started:**

1. **Clone the repository:**
   ```bash
   git clone https://github.com/mishraji874/Blockchain-in-Rust.git
   ```
2. **Navigate to the project directory:**
   ```bash
   cd Blockchain-in-Rust
   ```
3. **Now, build and run the project:**
   ```bash
   cargo build
   cargo run
   ```

## Contributing:

Contributions are welcome! Please feel free to submit pull requests or raise issues on the GitHub repository.