use block_chain::models::block::{Block, Mining};
use block_chain::models::blockchain::Blockchain;
use block_chain::models::{transaction::Transaction, wallet::Wallet};
use chrono::Utc;
use colored::Colorize;
use std::{
    io::{self, Write},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    // Clear the terminal and write the current time
    std::process::Command::new("clear").status().unwrap();
    print!(
        "{}-->",
        Utc::now()
            .format("Time: %H:%M:%S")
            .to_string()
            .yellow()
            .bold()
            .underline()
    );

    // Set the difficulty
    let difficulty = 4;

    // Create a new blockchain
    let mut blockchain = Blockchain::new(difficulty);
    println!("Blockchain with Genesis block is created");

    // Start a thread for displaying dots
    let _dot_thread = thread::spawn(|| {
        loop {
            thread::sleep(Duration::new(1, 0)); // Sleep for 1 second
            print!(
                "\r{}-->",
                Utc::now()
                    .format("Time: %H:%M:%S")
                    .to_string()
                    .yellow()
                    .bold()
                    .underline()
            );
            io::stdout().flush().unwrap(); // Flush the output to ensure the dot is displayed
        }
    });

    // Load wallets from the JSON file
    let wallets = Wallet::get_wallets().unwrap();

    // Set the wallets
    let mut wallet1 = wallets.get(&"Alice".to_owned()).unwrap().clone();
    let mut wallet2 = wallets.get(&"Bob".to_owned()).unwrap().clone();
    println!("\n2 wallets are loaded");

    // Simulation runs
    for simulate in 1..4 {
        println!(
            "{}{}{}",
            "Simulation Run: ".red(),
            simulate,
            " started...\n".red()
        );

        // Vector to store the valid transactions
        let mut transaction_pool: Vec<Transaction> = vec![];

        // Simulate transactions
        for index in 1..5 {
            // Create transaction
            let transaction = Transaction::new(
                format!("Sender: {}", wallet1.name()),
                format!("Recipient: {}", wallet2.name()),
                10.0 * index as f32,
                &wallet1,
            );

            // Check if the transaction amount is less than or equal to the sender wallet's balance
            if transaction.check_balance(&wallet1) {
                transaction_pool.push(transaction.clone());
            } else {
                let sender = transaction.sender().blue();
                let recipient = transaction.recipient().blue();
                let amount = transaction.amount().to_string().blue();
                let balance = wallet1.balance().to_string().red();
                let message = format!(
                    "{} --> {} --> Amount: {} --> Balance: {}",
                    sender, recipient, amount, balance
                );
                print!("{} --> ", message.red().bold());
                println!("{}", "Not enough balance".red().bold());
            }
        }

        if transaction_pool.len() > 0 {
            println!("{}", "Transactions are accepted".green());

            // Create a block with the transaction pool
            let block = Block::new(
                blockchain.chain.len(),
                transaction_pool.clone(),
                blockchain.chain.last().unwrap().hash.clone(),
            );

            // Shared atomic flag (mined,prof_of_work,consensus) for mining status
            let mining_data = Arc::new(Mutex::new(Mining::new()));

            // Spawn mining threads
            let nodes = 10;
            let threads_pool: Vec<_> = (0..nodes)
                .map(|i| {
                    // Clone the block and the atomic flag
                    let difficulty = difficulty.clone();
                    let mut clone_block = block.clone();
                    let mining_data = Arc::clone(&mining_data);

                    thread::spawn(move || {
                        println!("{}{}{}", "Thread no: ".purple(), i, " started mining");

                        loop {
                            clone_block.mine(difficulty, 10000000 * (i + 1) as u64);
                            if mining_data.lock().unwrap().mined {
                                break;
                            }

                            if !mining_data.lock().unwrap().mined {
                                mining_data.lock().unwrap().mined = true; // Set mined status to true

                                println!(
                                    "{}{}{}{}",
                                    "Thread no: ".purple(),
                                    i,
                                    " successfully mined a block. Proof of work: ".purple(),
                                    clone_block.proof_of_work
                                );

                                if mining_data.lock().unwrap().hash != String::default() {
                                    break;
                                } else {
                                    mining_data.lock().unwrap().hash = clone_block.hash.clone();
                                    if mining_data.lock().unwrap().proof_of_work != 0 {
                                        break;
                                    } else {
                                        mining_data.lock().unwrap().proof_of_work =
                                            clone_block.proof_of_work;
                                    }
                                }
                            }
                        }
                    })
                })
                .collect();

            // Once the block is mined, update the blockchain with verified block and wallet balances
            loop {
                if mining_data.lock().unwrap().mined
                    && mining_data.lock().unwrap().proof_of_work != 0
                {
                    let mut mined_block = block.clone();
                    mined_block.hash = mining_data.lock().unwrap().hash.clone();
                    mined_block.proof_of_work = mining_data.lock().unwrap().proof_of_work;

                    if mined_block.verify() {
                        println!("{}", "Block verified".yellow().bold());
                        blockchain.add_block(mined_block);

                        for transaction in transaction_pool.iter() {
                            wallet1.update_balance(transaction.amount());
                            wallet2.update_balance(transaction.amount() * -1.0);
                        }
                        println!(
                            "Wallet 1 - Remaining balance: {}",
                            wallet1.balance().to_string().red()
                        );
                        println!(
                            "Wallet 2 - Remaining balance: {}",
                            wallet2.balance().to_string().green()
                        );
                        blockchain
                            .write_to_file()
                            .expect("Failed to save blockchain to file");
                    }
                    break;
                }
            }

            // Wait for all threads to complete
            for thread in threads_pool {
                thread.join().unwrap();
            }
        } else {
            println!("{}", "No transaction accepted".red());
        }
    }

    // Blockchain is displayed as updated
    println!("{:?}", blockchain);
}
