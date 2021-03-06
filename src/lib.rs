use serde::{Deserialize, Serialize};
use serde::{Deserializer, Serializer};

use indexmap::IndexMap;
use sha2::{Digest, Sha256};

pub const TTL: usize = 16;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Request {
    NewBlockchain,
    LoadBlockchain { json: String },
    AddBlock { birth_hash: Hash, data: Vec<u8> },
    SpawnBlock { birth_data: Vec<u8>, data: Vec<u8> },
    GetTTL { birth_hash: Hash },
    GetEvents { birth_hash: Hash },
    Print,
    PrintGenesis,
    Quit,
}

impl std::fmt::Display for Request {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            fmt,
            "{}",
            serde_json::to_string(self).expect("Could not serialize Request, this is a bug.")
        )
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Default)]
pub struct Hash(Vec<u8>);

impl std::fmt::Display for Hash {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", &hex::encode(&self.0))
    }
}

impl Serialize for Hash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&hex::encode(&self.0))
    }
}

impl<'de> Deserialize<'de> for Hash {
    fn deserialize<D>(deserializer: D) -> Result<Hash, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = <String>::deserialize(deserializer)?;
        Ok(Hash(hex::decode(&s).unwrap()))
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
        birth_hash: Hash,
        data: Vec<u8>,
        preceding_hash: Option<Hash>,
    ) -> Self {
        let preceding_hash = preceding_hash.unwrap_or_default();
        let (birth_hash, hash) = if birth_hash.0.is_empty() {
            (
                Self::compute_hash(index, timestamp, &data, &preceding_hash.0),
                Self::compute_hash(index, timestamp, &data, &preceding_hash.0),
            )
        } else {
            let amalg = birth_hash
                .0
                .iter()
                .cloned()
                .chain(data.iter().cloned())
                .collect::<Vec<u8>>();
            let hash = Self::compute_hash(index, timestamp, &amalg, &preceding_hash.0);
            (birth_hash, hash)
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

    pub fn spawn(
        index: usize,
        timestamp: u64,
        birth_data: Vec<u8>,
        data: Vec<u8>,
        preceding_hash: Option<Hash>,
    ) -> Self {
        let preceding_hash = preceding_hash.unwrap_or_default();
        let (birth_hash, hash) = if birth_data.is_empty() {
            (
                Self::compute_hash(index, timestamp, &data, &preceding_hash.0),
                Self::compute_hash(index, timestamp, &data, &preceding_hash.0),
            )
        } else {
            let amalg = birth_data
                .into_iter()
                .chain(data.iter().cloned())
                .collect::<Vec<u8>>();
            let birth_hash = Self::compute_hash(index, timestamp, &amalg, &preceding_hash.0);
            (birth_hash.clone(), birth_hash)
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
        Hash(hasher.finalize().to_vec())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Blockochen {
    pub genesis_hash: Hash,
    pub chen: IndexMap<Hash, Blocko>,
    pub ts: u64,
}

impl Blockochen {
    pub fn new() -> Self {
        let gen_blocko = Blocko::new(
            0,
            0,
            Hash(vec![]),
            b"Initial Block in the Chain".to_vec(),
            Some(Hash(b"0".to_vec())),
        );
        Self {
            genesis_hash: gen_blocko.hash.clone(),
            chen: [(gen_blocko.hash.clone(), gen_blocko)].into(),
            ts: 0,
        }
    }

    pub fn last(&self) -> &Blocko {
        self.chen.last().map(|(_, b)| b).unwrap()
    }

    pub fn add(&mut self, birth_hash: Hash, data: Vec<u8>) -> Result<Hash, Option<usize>> {
        if birth_hash != self.genesis_hash {
            match self.get_ttl(birth_hash.clone()) {
                t @ None | t @ Some(0) => return Err(t),
                _ => {}
            }
        }
        let timestamp = self.ts;
        self.ts += 1;
        let index = self.chen.len();
        let blocko = Blocko::new(
            index,
            timestamp,
            birth_hash,
            data,
            Some(self.last().hash.clone()),
        );
        let ret = blocko.hash.clone();
        self.chen.insert(blocko.hash.clone(), blocko);
        Ok(ret)
    }

    pub fn spawn(&mut self, birth_data: Vec<u8>, data: Vec<u8>) -> Hash {
        let timestamp = self.ts;
        self.ts += 1;
        let index = self.chen.len();
        let blocko = Blocko::spawn(
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
        self.chen.get_index(index).map(|(h, _)| h.0.as_slice())
    }

    pub fn get_events(&self, birth_hash: Hash) -> Option<Vec<Vec<u8>>> {
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

    pub fn get_ttl(&self, birth_hash: Hash) -> Option<usize> {
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
