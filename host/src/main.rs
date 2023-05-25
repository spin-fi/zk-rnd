// TODO: Update the name of the method loaded by the prover. E.g., if the method
// is `multiply`, replace `METHOD_NAME_ELF` with `MULTIPLY_ELF` and replace
// `METHOD_NAME_ID` with `MULTIPLY_ID`
use borsh::{BorshDeserialize, BorshSerialize};
use methods::{METHOD_NAME_ELF, METHOD_NAME_ID};
use risc0_zkvm::serde::{from_slice, to_vec};
use risc0_zkvm::sha::Digest;
use risc0_zkvm::{Executor, ExecutorEnv};

#[derive(BorshSerialize, BorshDeserialize)]
pub enum Action {
    Call,
}

fn main() {
    println!("starting...");
    let state: Vec<(Vec<u8>, Vec<u8>)> = vec![];
    let actions = vec![Action::Call, Action::Call];
    let mut row_data = Vec::new();
    (state, actions).serialize(&mut row_data).unwrap();
    println!("send data {:?}", row_data.as_slice());
    let env = ExecutorEnv::builder()
        .add_input(&to_vec(row_data.as_slice()).unwrap())
        .build();
    let mut exec = Executor::from_elf(env, METHOD_NAME_ELF).unwrap();
    println!("run execution...");
    let session = exec.run().unwrap();
    println!("prove execution...");
    let receipt = session.prove().unwrap();

    // TODO: Implement code for transmitting or serializing the receipt for
    // other parties to verify here
    println!("journal {}", &receipt.journal.len(),);
    println!(
        "from {:x?}\nto   {:x?}",
        hex::encode(&receipt.journal[0..32]),
        hex::encode(&receipt.journal[32..]),
        //from_slice::<(Digest, Digest), _>(&receipt.journal).unwrap()
    );

    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    println!("verify execution...");
    receipt.verify(METHOD_NAME_ID).unwrap();
}
