extern crate alloc;
use alloc::{string::String, sync::Arc};
use borsh::{BorshDeserialize, BorshSerialize};
use spin::Mutex;

use crate::zk::ZkStorage;

pub trait Storage {
    fn read<K: BorshSerialize, V: BorshDeserialize>(&self, key: K) -> Option<V>;
    fn write<K: BorshSerialize, V: BorshSerialize + BorshDeserialize>(
        &mut self,
        key: K,
        value: V,
    ) -> Option<V>;
}

pub trait Logger {
    fn log<L: BorshSerialize>(&mut self, log: &L);
}

pub trait Metadata {
    fn timestamp(&self) -> u64;
}

pub trait Action<T: BorshDeserialize> {
    fn body(&self) -> T;
    fn sender(&self) -> String;
}

pub trait Handler {
    fn handle<B: BorshDeserialize, /*S: Storage,*/ A: Action<B>, L: Logger, M: Metadata>(
        // storage: &mut S,
        storage: Arc<Mutex<ZkStorage>>,
        action: &A,
        logger: &mut L,
        metadata: &M,
    );
}
