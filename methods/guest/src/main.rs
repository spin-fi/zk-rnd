#![no_main]
#![no_std]

extern crate alloc;

use alloc::format;
use alloc::vec::Vec;
use risc0_zkvm::guest::env;
use zk_sdk::{interface::Handler, actions::ZkAction, zk::{ZkStorage, ZkLogger, ZkMetadata}};
use dex::handler::DexHandler;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let row_data: Vec<u8> = env::read();
    env::log(&format!("data {:?}", row_data));
    let (storage_values, actions): (Vec<(Vec<u8>, Vec<u8>)>, Vec<ZkAction>) =
        borsh::BorshDeserialize::deserialize(&mut row_data.as_slice()).unwrap();
    let mut storage = ZkStorage::new(storage_values);
    let mut logger = ZkLogger {};
    let metadata = ZkMetadata { ts: 0 };

    let original_hash = storage.compute_root();

    for action in actions {
        DexHandler::handle(&mut storage, &action, &mut logger, &metadata);
    }

    let new_hash = storage.compute_root();

    env::commit_slice(&original_hash);
    env::commit_slice(&new_hash);
}
