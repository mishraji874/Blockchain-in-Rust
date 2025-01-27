use super::block::Block;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{fs::OpenOptions, io::Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let mut genesis_block = Block::new(0, vec![], String::default());
        let genesis_block_difficulty = 4;
        genesis_block.mine(genesis_block_difficulty, 0);
        let chain = vec![genesis_block.clone()];
        Self { chain, difficulty }
    }

    pub fn add_block(&mut self, block: Block) {
        self.chain.push(block);
    }

    pub fn write_to_file(&self) -> Result<()> {
        let file_path = "w_block_chain.json";
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }
}
