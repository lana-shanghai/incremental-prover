use anyhow::Result;
use risc0_zkvm::sha::Digest;
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};
// use incremental_methods::{AGGREGATOR_ELF, AGGREGATOR_ID};
use bincode;
use tracing::info;

pub const AGGREGATOR_ELF: &[u8] = include_bytes!("/Users/lana/github/incremental-prover/methods/target/riscv-guest/incremental-methods/aggregator/riscv32im-risc0-zkvm-elf/release/aggregator.bin");
pub const AGGREGATOR_ID: [u32; 8] = [
    3359609298, 4220649381, 2187998162, 2646582995, 3559333735, 2905452592, 381574800, 2560628512,
];
pub const KAILUA_FPVM_ID: [u32; 8] = [
    3119893061, 2377629108, 1437993944, 394392861, 3943328219, 1766698319, 2692453892, 715568627,
];

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "risc0_zkvm=debug".into()))
        .init();

    println!("Length of aggregator elf: {:?}", AGGREGATOR_ELF.len());

    let aggregated_id = Digest::new(AGGREGATOR_ID);
    println!("Aggregated ID: {:?}", aggregated_id);

    // succinct proof of block 15154000
    let block_bytes: &[u8] = include_bytes!(
        "./risc0-2.1.0-0x1bf48d54157dd4b07ff0ba429a295d3be6e82d0553f6d04adba4e3e663837c65.zkp"
    );
    let block_receipt: Receipt = bincode::deserialize(block_bytes)?;
    let journal_bytes = block_receipt.journal.as_ref();

    let env = ExecutorEnv::builder()
        .add_assumption(block_receipt.clone())
        .write(&journal_bytes)
        .unwrap()
        .build()?;

    println!("Added assumption");

    let result = default_prover().prove(env, AGGREGATOR_ELF);

    match result {
        Ok(session) => {
            println!("Prover succeeded");
            let receipt = session.receipt;
            println!("Receipt created");
            receipt.verify(aggregated_id).unwrap();
            println!("Verified aggregated receipt");
        }
        Err(err) => {
            eprintln!("Prover failed: {:#?}", err);
            std::process::exit(1);
        }
    }

    Ok(())
}
