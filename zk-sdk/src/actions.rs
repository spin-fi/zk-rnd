use alloc::string::String;
use borsh::{BorshDeserialize, BorshSerialize};

use crate::interface::Action;

#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub enum ZkActionBody {
    Call,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct ZkAction {
    action: ZkActionBody,
    sender: String,
}

impl ZkAction {
    pub fn new(action: ZkActionBody, sender: String) -> Self {
        ZkAction {
            action,
            sender: sender.clone(),
        }
    }
}

impl Action<ZkActionBody> for ZkAction {
    fn body(&self) -> ZkActionBody {
        self.action.clone()
    }
    fn sender(&self) -> String {
        self.sender.clone()
    }
}
