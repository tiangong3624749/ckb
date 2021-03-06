use bigint::{H256, U256};
use ckb_core::block::Block;
use ckb_core::header::{BlockNumber, Header, RawHeader, Seal};

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    // genesis data
    pub version: u32,
    pub parent_hash: H256,
    pub timestamp: u64,
    pub txs_commit: H256,
    pub difficulty: U256,
    pub number: BlockNumber,
    pub nonce: u64,
    pub proof: Vec<u8>,
    // other config
    pub initial_block_reward: Capacity,
}

impl Config {
    pub fn default() -> Self {
        Config {
            version: 0,
            parent_hash: H256::from(0),
            timestamp: 0,
            txs_commit: H256::from(0),
            difficulty: U256::from(0),
            number: 0,
            nonce: 0,
            proof: Vec::new(),
            initial_block_reward: 0,
        }
    }

    pub fn genesis_block(&self) -> Block {
        let header = Header {
            raw: RawHeader {
                version: self.version,
                parent_hash: self.parent_hash,
                timestamp: self.timestamp,
                txs_commit: self.txs_commit,
                difficulty: self.difficulty,
                number: self.number,
            },
            seal: Seal {
                nonce: self.nonce,
                proof: self.proof,
            },
        };

        Block {
            header: header.into(),
            transactions: vec![],
        }
    }
}
