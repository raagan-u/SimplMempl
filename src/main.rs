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

// co-ordinator that validates submitted txs, and triggers mempool's state transitions
pub struct Node {
    mempool: Mempool,
    blockchain: Blockchain
}

fn main() {
    println!("Hello, world!");
}
