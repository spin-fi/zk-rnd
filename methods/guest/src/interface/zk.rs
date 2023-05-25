extern crate alloc;

use alloc::vec::Vec;
use borsh::BorshDeserialize;
use hashbrown::HashMap;
//use risc0_zkvm::guest::env;
use risc0_zkvm::sha::{Impl, Sha256};
use rs_merkle::Hasher;
use rs_merkle::MerkleTree;

#[derive(Clone)]
pub struct ZkSha;

impl Hasher for ZkSha {
    type Hash = [u8; 32];

    fn hash(data: &[u8]) -> Self::Hash {
        let dig = Impl::hash_bytes(data);
        let bytes = dig.as_bytes();
        <[u8; 32]>::try_from(bytes).unwrap()
    }
}

use super::*;

#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub enum ZkActionBody {
    Call,
}

pub struct ZkAction {
    action: ZkActionBody,
    sender: String,
}

impl Action<ZkActionBody> for ZkAction {
    fn body(&self) -> ZkActionBody {
        self.action.clone()
    }
    fn sender(&self) -> String {
        self.sender()
    }
}

pub struct ZkStorage {
    pub storage: HashMap<Vec<u8>, Vec<u8>>,
}

impl ZkStorage {
    pub fn new(values: Vec<(Vec<u8>, Vec<u8>)>) -> Self {
        Self {
            storage: HashMap::from_iter(values.into_iter()),
        }
    }

    pub fn compute_root(&self) -> [u8; 32] {
        //thin place is that (k,v) -> into bytes -> into hash might be non deterministinc
        let hashed_values: Vec<[u8; 32]> = self
            .storage
            .iter()
            .map(|x| {
                let mut bytes = x.0.clone();
                bytes.append(&mut x.1.clone());
                ZkSha::hash(bytes.as_slice())
            })
            .collect();
        if !hashed_values.is_empty() {
            let merkle_tree = MerkleTree::<ZkSha>::from_leaves(&hashed_values);
            merkle_tree.root().expect("can't compute merkle root")
        } else {
            [0u8; 32]
        }
    }
}

impl Storage for ZkStorage {
    fn read<K: borsh::BorshSerialize, V: borsh::BorshDeserialize>(&self, key: K) -> Option<V> {
        let mut k = Vec::new();
        key.serialize(&mut k).expect("can't serialize key");
        self.storage
            .get(&k)
            .map(|bytes| BorshDeserialize::deserialize(&mut bytes.as_slice()).ok())
            .flatten()
    }
    fn write<K: borsh::BorshSerialize, V: borsh::BorshSerialize + borsh::BorshDeserialize>(
        &mut self,
        key: K,
        value: V,
    ) -> Option<V> {
        let mut k = Vec::new();
        key.serialize(&mut k).expect("can't serialize key");
        let mut v = Vec::new();
        value.serialize(&mut v).expect("can't serialize value");
        self.storage
            .insert(k, v)
            .map(|bytes| BorshDeserialize::deserialize(&mut bytes.as_slice()).ok())
            .flatten()
    }
}

pub struct ZkLogger;

impl Logger for ZkLogger {
    fn log<L: BorshSerialize>(&mut self, _log: &L) {
        // write no logs while zk execution
    }
}

pub struct ZkMetadata {
    pub ts: u64,
}

impl Metadata for ZkMetadata {
    fn timestamp(&self) -> u64 {
        self.ts
    }
}
