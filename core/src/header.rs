use bigint::{H256, U256};
use bincode::{deserialize, serialize};
use hash::sha3_256;

pub use BlockNumber;

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug, Default)]
pub struct Seal {
    nonce: u64,
    proof: Vec<u8>,
}

impl Seal {
    pub fn new(nonce: u64, proof: Vec<u8>) -> Self {
        Seal { nonce, proof }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug, Default)]
pub struct RawHeader {
    version: u32,
    /// Parent hash.
    parent_hash: H256,
    /// Block timestamp(ms).
    timestamp: u64,
    /// Genesis number is 0, Child block number is parent block number + 1.
    number: BlockNumber,
    /// Transactions merkle root.
    txs_commit: H256,
    /// Transactions proposal merkle root.
    txs_proposal: H256,
    /// Block difficulty.
    difficulty: U256,
    /// Hash of the cellbase
    cellbase_id: H256,
    /// Hash of the uncles
    uncles_hash: H256,
    /// Hash of the uncles
    uncles_count: u32,
}

impl RawHeader {
    pub fn pow_hash(&self) -> H256 {
        sha3_256(serialize(self).unwrap()).into()
    }

    pub fn with_seal(self, seal: Seal) -> Header {
        let builder = HeaderBuilder {
            inner: Header {
                raw: self,
                seal,
                hash: H256::zero(),
            },
        };
        builder.build()
    }

    pub fn number(&self) -> BlockNumber {
        self.number
    }

    pub fn difficulty(&self) -> U256 {
        self.difficulty
    }

    pub fn uncles_count(&self) -> u32 {
        self.uncles_count
    }

    pub fn mut_uncles_count(&mut self) -> &mut u32 {
        &mut self.uncles_count
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
pub struct Header {
    raw: RawHeader,
    /// proof seal
    seal: Seal,
    #[serde(skip)]
    hash: H256,
}

impl Header {
    pub fn version(&self) -> u32 {
        self.raw.version
    }

    pub fn number(&self) -> BlockNumber {
        self.raw.number
    }

    pub fn difficulty(&self) -> U256 {
        self.raw.difficulty
    }

    pub fn timestamp(&self) -> u64 {
        self.raw.timestamp
    }

    pub fn proof(&self) -> &[u8] {
        &self.seal.proof
    }

    pub fn nonce(&self) -> u64 {
        self.seal.nonce
    }

    pub fn hash(&self) -> H256 {
        self.hash
    }

    pub fn pow_hash(&self) -> H256 {
        self.raw.pow_hash()
    }

    pub fn is_genesis(&self) -> bool {
        self.number() == 0
    }

    pub fn parent_hash(&self) -> H256 {
        self.raw.parent_hash
    }

    pub fn txs_commit(&self) -> H256 {
        self.raw.txs_commit
    }

    pub fn txs_proposal(&self) -> H256 {
        self.raw.txs_proposal
    }

    pub fn cellbase_id(&self) -> H256 {
        self.raw.cellbase_id
    }

    pub fn uncles_hash(&self) -> H256 {
        self.raw.uncles_hash
    }

    pub fn into_raw(self) -> RawHeader {
        self.raw
    }

    pub fn uncles_count(&self) -> u32 {
        self.raw.uncles_count
    }
}

#[derive(Default)]
pub struct HeaderBuilder {
    inner: Header,
}

impl HeaderBuilder {
    pub fn new(bytes: &[u8]) -> Self {
        HeaderBuilder {
            inner: deserialize(bytes).expect("header deserializing should be ok"),
        }
    }

    pub fn header(mut self, header: Header) -> Self {
        self.inner = header;
        self
    }

    pub fn version(mut self, version: u32) -> Self {
        self.inner.raw.version = version;
        self
    }

    pub fn number(mut self, number: BlockNumber) -> Self {
        self.inner.raw.number = number;
        self
    }

    pub fn difficulty(mut self, difficulty: &U256) -> Self {
        self.inner.raw.difficulty = *difficulty;
        self
    }

    pub fn timestamp(mut self, timestamp: u64) -> Self {
        self.inner.raw.timestamp = timestamp;
        self
    }

    pub fn proof(mut self, proof: &[u8]) -> Self {
        self.inner.seal.proof = proof.to_vec();
        self
    }

    pub fn nonce(mut self, nonce: u64) -> Self {
        self.inner.seal.nonce = nonce;
        self
    }

    pub fn parent_hash(mut self, hash: &H256) -> Self {
        self.inner.raw.parent_hash = *hash;
        self
    }

    pub fn txs_commit(mut self, hash: &H256) -> Self {
        self.inner.raw.txs_commit = *hash;
        self
    }

    pub fn txs_proposal(mut self, hash: &H256) -> Self {
        self.inner.raw.txs_proposal = *hash;
        self
    }

    pub fn cellbase_id(mut self, hash: &H256) -> Self {
        self.inner.raw.cellbase_id = *hash;
        self
    }

    pub fn uncles_hash(mut self, hash: &H256) -> Self {
        self.inner.raw.uncles_hash = *hash;
        self
    }

    pub fn uncles_count(mut self, uncles_count: u32) -> Self {
        self.inner.raw.uncles_count = uncles_count;
        self
    }

    pub fn build(self) -> Header {
        let hash = H256::from_slice(&sha3_256(serialize(&self.inner).unwrap()));
        self.with_hash(&hash)
    }

    pub fn with_hash(self, hash: &H256) -> Header {
        let mut header = self.inner;
        header.hash = *hash;
        header
    }
}
