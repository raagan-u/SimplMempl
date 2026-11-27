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
        let tx_id_check = Box::new(| tx : &Transaction| !tx.id.is_empty());
        ValidatorPipeline { validator_funcs: vec![tx_id_check] }
    }
}

impl ValidatorPipeline<Transaction> {
    pub fn check_tx(&self, tx: &Transaction) -> Result<bool, NodeError> {
        for i in &self.validator_funcs {
            if i(tx) == false {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

#[derive(Debug, Error)]
pub enum NodeError {
    #[error("submitted tx is invalid: {0}")]
    InvalidTxID(String)
}

impl Node {
    pub fn new(validator: ValidatorPipeline<Transaction>, mempool: Mempool, blockchain: Blockchain) -> Node {
        Self { validator, mempool, blockchain }
    }
    
    pub fn submit_tx(&mut self, tx: Transaction) -> Result<TxStatus, NodeError> {
        let result = self.validator.check_tx(&tx);
        match result {
            Ok(true) => {
                self.mempool.submit_tx(tx);
                return Ok(TxStatus::Accpeted)
            },
            Ok(false) => return Ok(TxStatus::Rejected),
            Err(e) => return Err(e)
        }
    }

    pub fn tx_status(&self, id: &str) -> Option<TxStatus> {
        self.mempool.tx_status(id)
    }
}