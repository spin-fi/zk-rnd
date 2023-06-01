use borsh::BorshSerialize;
use methods::{GUEST_METHOD_ELF, GUEST_METHOD_ID};
use risc0_zkvm::serde::to_vec;
use risc0_zkvm::{Executor, ExecutorEnv};
use zk_sdk::actions::{ZkAction, ZkActionBody};

fn main() {
    println!("starting...");

    let state: Vec<(Vec<u8>, Vec<u8>)> = vec![];
    let actions = vec![ZkAction::new(ZkActionBody::Call, "host".to_string())];
    let mut row_data = Vec::new();
    (state, actions).serialize(&mut row_data).unwrap();
    println!("send data {:?}", row_data.as_slice());

    let env = ExecutorEnv::builder()
        .add_input(&to_vec(row_data.as_slice()).unwrap())
        .build();
    let mut exec = Executor::from_elf(env, GUEST_METHOD_ELF).unwrap();

    let start = std::time::Instant::now();
    println!("run execution...");
    let session = exec.run().unwrap();
    println!("executed in {:?}", start.elapsed());

    let start = std::time::Instant::now();
    println!("prove execution...");
    let receipt = session.prove().unwrap();
    println!("proved in {:?}", start.elapsed());

    println!(
        "from {:x?}\nto   {:x?}",
        hex::encode(&receipt.journal[0..32]),
        hex::encode(&receipt.journal[32..]),
        //from_slice::<(Digest, Digest), _>(&receipt.journal).unwrap()
    );

    let start = std::time::Instant::now();
    println!("verify execution...");
    receipt.verify(GUEST_METHOD_ID).unwrap();
    println!("verified in {:?}", start.elapsed());

    println!(
        "receipt: {} bytes",
        (receipt.segments.get(0).unwrap().get_seal_bytes().len() as f32 / 1024.0) as u32
    );

    assert_eq!(receipt.segments.len(), 1);

    println!("output: {:?}", receipt.journal);
}
