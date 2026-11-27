use crate::node::{Node, ValidatorPipeline};

mod node;

#[derive(Debug, Clone)]
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

impl Blockchain {
    pub fn new() -> Blockchain {
        Self { blocks: vec![]}
    }
}

#[derive(Debug, Clone)]
pub struct Mempool {
    accepted_txs: Vec<Transaction>
}

impl Mempool {
    pub fn new() -> Mempool {
        Self {accepted_txs: Vec::new()}
    }

    pub fn submit_tx(&mut self, tx: Transaction) {
        self.accepted_txs.push(tx);
    }

    pub fn tx_status(&self, id: &str) -> Option<TxStatus> {
        let result = self.accepted_txs.iter().find(|i | i.id == id);
        if result.is_some() {
            return Some(TxStatus::Accpeted)
        }
        
        Some(TxStatus::Dropped) 
    }
}

#[derive(Debug)]
pub enum TxStatus {
    Accpeted,
    Rejected,
    Pending,
    Confirmed,
    Dropped
}


fn main() {
    println!("[+] Initializing SimplMempl......");

    let mempool = Mempool::new();
    let blockchain = Blockchain::new();
    let validator_pipeline = ValidatorPipeline::default();
    let mut node = Node::new(validator_pipeline, mempool, blockchain);

    println!("[*] Node initialized and ready to accept transactions");

    println!("[+] creating intentional empty id tx expecting rejection");
    let empty_tx = Transaction{ id: "".to_string(), fee: 0, weight: 0};
    let submission_result = node.submit_tx(empty_tx);
    println!("submission result {:#?}", submission_result);

    println!("[+] creating non empty id tx expecting rejection");
    let empty_tx = Transaction{ id: "tx1234".to_string(), fee: 0, weight: 0};
    let submission_result = node.submit_tx(empty_tx);
    println!("submission result {:#?}", submission_result);

    println!("{:#?}", node.tx_status("tx1234"));
}
