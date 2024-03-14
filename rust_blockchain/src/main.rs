use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize, Deserialize)]
struct Block {
    timestamp: i64,
    prev_block_hash: String,
    hash: String,
    data: String,
}

impl Block {
    fn new(data: String, prev_block_hash: String) -> Self {
        let timestamp = Utc::now().timestamp();
        let mut block = Block {
            timestamp,
            prev_block_hash,
            hash: String::new(),
            data,
        };
        block.hash = block.calculate_hash();
        block
    }

    fn calculate_hash(&self) -> String {
        let headers = format!("{}{}{}", self.prev_block_hash, self.timestamp, self.data);
        let mut hasher = Sha256::new();
        hasher.update(headers);
        let hash = hasher.finalize();
        format!("{:x}", hash)
    }
}

#[derive(Debug)]
struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        let genesis_block = Block::new("Genesis Block".to_string(), String::new());
        Blockchain {
            blocks: vec![genesis_block],
        }
    }

    fn add_block(&mut self, data: String) {
        let prev_block = self.blocks.last().unwrap();
        let new_block = Block::new(data, prev_block.hash.clone());
        self.blocks.push(new_block);
    }
}

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block("Block 1 Data".to_string());
    blockchain.add_block("Block 2 Data".to_string());
    blockchain.add_block("Block 3 Data".to_string());
    blockchain.add_block("Block 4 Data".to_string());
    blockchain.add_block("Block 5 Data".to_string());
    blockchain.add_block("Block 6 Data".to_string());
    blockchain.add_block("Block 7 Data".to_string());
    blockchain.add_block("Block 8 Data".to_string());
    blockchain.add_block("Block 9 Data".to_string());
    blockchain.add_block("Block 10 Data".to_string());

    for block in blockchain.blocks {
        println!("{:?}", block);
    }
}
