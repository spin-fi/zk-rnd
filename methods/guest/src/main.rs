#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std] // std support is experimental

extern crate alloc;
pub mod interface;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use borsh::BorshDeserialize;
use interface::zk::*;
use interface::{Action, Handler};

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub struct HandleThings;

impl Handler for HandleThings {
    fn handle<
        B: BorshDeserialize,
        S: interface::Storage,
        A: interface::Action<B>,
        L: interface::Logger,
        M: interface::Metadata,
    >(
        storage: &mut S,
        _action: &A,
        _logger: &mut L,
        _metadata: &M,
    ) {
        storage.write("key", String::from("value"));
        storage.write("key1", String::from("value"));
        storage.write("key2", String::from("value"));
    }
}

pub fn main() {
    // TODO: Implement your guest code here
    let row_data: Vec<u8> = env::read();
    env::log(&format!("data {:?}", row_data));
    let (storage_values, actions): (Vec<(Vec<u8>, Vec<u8>)>, Vec<TestAction>) =
        borsh::BorshDeserialize::deserialize(&mut row_data.as_slice()).unwrap();
    let mut storage = ZkStorage::new(storage_values);
    let mut logger = ZkLogger {};
    let metadata = ZkMetadata { ts: 0 };

    let original_hash = storage.compute_root();

    for action in actions {
        HandleThings::handle(&mut storage, &action, &mut logger, &metadata);
    }

    let new_hash = storage.compute_root();
    env::log(&format!("root {:?}", new_hash));
    env::log(&format!("{:?}", storage.storage));

    env::commit_slice(&original_hash);
    env::commit_slice(&new_hash);
}
