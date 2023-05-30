#![no_main]
#![no_std]

extern crate alloc;

use core::cell::RefCell;
use alloc::string::{String, ToString};
use alloc::{format, sync::Arc};
use alloc::vec::Vec;
use borsh::BorshDeserialize;
use risc0_zkvm::guest::env;
use spin::Mutex;
use zk_sdk::{interface::Handler, actions::ZkAction, zk::{ZkStorage, ZkLogger, ZkMetadata}};
use dex::handler::DexHandler;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let row_data: Vec<u8> = env::read();
    env::log(&format!("data {:?}", row_data));
    let (storage_values, actions): (Vec<(Vec<u8>, Vec<u8>)>, Vec<ZkAction>) =
        borsh::BorshDeserialize::deserialize(&mut row_data.as_slice()).unwrap();
    
    let storage = ZkStorage::new(storage_values);
    let original_hash = storage.compute_root();
    
    let mut logger = ZkLogger { data: Vec::new() };
    let metadata = ZkMetadata { ts: 0 };

    let boxed_storage = Arc::new(Mutex::new(storage));

    for action in actions {
        DexHandler::handle(boxed_storage.clone(), &action, &mut logger, &metadata);
    }

    // console debug development
    for entry in logger.data {
        let string: String = BorshDeserialize::deserialize(&mut &entry[..]).unwrap();
        env::log(&string);
    }

    let new_hash = boxed_storage.lock().compute_root();

    env::commit_slice(&original_hash);
    env::commit_slice(&new_hash);
}
