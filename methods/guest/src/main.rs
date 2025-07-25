// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use risc0_zkvm::{guest::env, Receipt};
use bincode;
use kailua_common::journal::ProofJournal;

pub const KAILUA_FPVM_ID: [u32; 8] = [3119893061, 2377629108, 1437993944, 394392861, 3943328219, 1766698319, 2692453892, 715568627];

fn main() {
    let journal: Vec<u8> = env::read();
    
    println!("Verifying Kailua journal...");
    env::verify(KAILUA_FPVM_ID, &journal);
    println!("Verified Kailua journal");
    
    let mut block_journal = ProofJournal::decode_packed(&journal);
    block_journal.claimed_l2_block_number += 2;
    
    env::commit_slice(&block_journal.encode_packed());
}
