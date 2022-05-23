use indexmap::IndexMap;
use sha2::{Digest, Sha256};

pub type Hash = Vec<u8>;

pub const TTL: usize = 16;

#[derive(Debug)]
pub struct Blocko {
    pub index: usize,
    pub timestamp: u64,
    pub data: Vec<u8>,
    pub birth_hash: Hash,
    pub preceding_hash: Hash,
    pub hash: Hash,
}

impl Blocko {
    pub fn new(
        index: usize,
        timestamp: u64,
        birth_data: Vec<u8>,
        data: Vec<u8>,
        preceding_hash: Option<Hash>,
    ) -> Self {
        let preceding_hash = preceding_hash.unwrap_or_default();
        let (birth_hash, hash) = if birth_data.is_empty() {
            (
                Self::compute_hash(index, timestamp, data.as_slice(), preceding_hash.as_slice()),
                Self::compute_hash(index, timestamp, data.as_slice(), preceding_hash.as_slice()),
            )
        } else {
            let hash = Self::compute_hash(
                index,
                timestamp,
                birth_data.as_slice(),
                preceding_hash.as_slice(),
            );
            (birth_data, hash)
        };
        Self {
            index,
            timestamp,
            birth_hash,
            data,
            preceding_hash,
            hash,
        }
    }

    pub fn compute_hash(index: usize, timestamp: u64, data: &[u8], preceding_hash: &[u8]) -> Hash {
        let mut hasher = Sha256::new();
        hasher.update(index.to_be_bytes());
        hasher.update(timestamp.to_be_bytes());
        hasher.update(data);
        hasher.update(preceding_hash);
        hasher.finalize().to_vec()
    }
}

#[derive(Debug)]
pub struct Blockochen {
    pub chen: IndexMap<Hash, Blocko>,
    pub ts: u64,
}

impl Blockochen {
    pub fn new() -> Self {
        let gen_blocko = Blocko::new(
            0,
            0,
            vec![],
            b"Initial Block in the Chain".to_vec(),
            Some(b"0".to_vec()),
        );
        Self {
            chen: [(gen_blocko.hash.clone(), gen_blocko)].into(),
            ts: 0,
        }
    }

    pub fn last(&self) -> &Blocko {
        self.chen.last().map(|(_, b)| b).unwrap()
    }

    pub fn add(&mut self, birth_data: Vec<u8>, data: Vec<u8>) -> Hash {
        let timestamp = self.ts;
        self.ts += 1;
        let index = self.chen.len();
        let blocko = Blocko::new(
            index,
            timestamp,
            birth_data,
            data,
            Some(self.last().hash.clone()),
        );
        let ret = blocko.hash.clone();
        self.chen.insert(blocko.hash.clone(), blocko);
        ret
    }

    pub fn get_hash(&self, index: usize) -> Option<&[u8]> {
        self.chen.get_index(index).map(|(h, _)| h.as_slice())
    }

    pub fn get_events(&self, birth_hash: &[u8]) -> Option<Vec<Vec<u8>>> {
        let ret: Vec<Vec<u8>> = self
            .chen
            .iter()
            .filter(|(_, b)| b.birth_hash == birth_hash)
            .map(|(_, b)| b.data.clone())
            .collect();
        if ret.is_empty() {
            None
        } else {
            Some(ret)
        }
    }

    pub fn get_ttl(&self, birth_hash: &[u8]) -> Option<usize> {
        if let Some((h, _)) = self
            .chen
            .iter()
            .rev()
            .find(|(_, b)| b.birth_hash == birth_hash)
        {
            self.chen
                .get_index_of(h)
                .map(|r| TTL.saturating_sub((self.chen.len()).saturating_sub(r)))
        } else {
            None
        }
    }
}
