use alloc::string::String;
use borsh::BorshDeserialize;
use zk_sdk::interface::{Action, Handler, Logger, Metadata, Storage};

pub struct DexHandler;

impl Handler for DexHandler {
    fn handle<B: BorshDeserialize, S: Storage, A: Action<B>, L: Logger, M: Metadata>(
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
