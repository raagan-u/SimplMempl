use thiserror::Error;

use crate::{Blockchain, Mempool, Transaction, TxStatus};

// co-ordinator that validates submitted txs, and triggers mempool's state transitions
pub struct Node {
    validator: ValidatorPipeline<Transaction>,
    mempool: Mempool,
    blockchain: Blockchain
}

pub struct ValidatorPipeline<T> {
    validator_funcs: Vec<Box<dyn Fn(&T) -> bool>>,
}

impl Default for ValidatorPipeline<Transaction> {
    fn default() -> Self {
        ValidatorPipeline { validator_funcs: vec![] }
    }
}

#[derive(Debug, Error)]
pub enum NodeError {
    #[error("submitted tx is invalid: {0}")]
    InvalidTxID(String)
}

impl Node {
    pub fn submit_tx(tx: Transaction) -> Result<TxStatus, NodeError> {
        
        Ok(TxStatus::Accpeted)
    }
}