use thiserror::Error;

mod node;

pub struct Transaction {
    id: String,
    fee: u64,
    weight: u64
}

pub const GENESIS: &str = "GENESIS_BLOCK";

pub struct Block {
    hash: String,
    parent_block_hash: String
}

pub struct Blockchain {
    blocks: Vec<Block>
}

pub struct Mempool {
    
}

pub enum TxStatus {
    Accpeted,
    Rejected,
    Pending,
    Confirmed,
    Dropped
}


fn main() {
    println!("Hello, world!");
}
