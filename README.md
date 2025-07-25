# incremental-prover
A prover that uses Kailua to create continuously aggregated proofs of OP stack based chain validity. 

### Build 

```
cd methods && cargo build -r && cargo risczero build && cd ..
```

The commands build the `guest` package in `methods` and creates the ELF file and ID for the guest program. 

```
use incremental_methods::{AGGREGATOR_ELF, AGGREGATOR_ID};
```

### Run 

From root run `cargo run -r`

### Flow

The host program loads a Kailua / Risc0 generated `Receipt` (aka proof) for a BOB Sepolia block. It passes it as an `assumption` to the guest program `env` where the block execution is verified against the Kailua proving service. The guest program applies new state changes to the `Journal` of the verified `Receipt` (currently a dummy mutation of adding two blocks to the L2 height) and commits to it. The host program proves the execution of the guest program and verifies the receipt against the guest ELF. 

### Further work

Build on top of the composition proving in order to achieve an aggregate chain prover. 