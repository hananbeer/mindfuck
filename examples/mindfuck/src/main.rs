// Copyright 2023 RISC Zero, Inc.
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

use mindfuck_core::Outputs;
use mindfuck_methods::{MINDFUCK_ELF, MINDFUCK_ID};
use risc0_zkvm::{
    serde::{from_slice, to_vec},
    Prover,
};

fn main() {
    let bf_program = include_str!("../res/exploit.bf");
    // let bf_input = include_str!("../res/input.bin");

    // Make the prover.
    let mut prover =
        Prover::new(MINDFUCK_ELF).expect("Prover should be constructed from valid ELF binary");

    prover.add_input_u32_slice(&to_vec(&bf_program).expect("should be serializable"));
    // prover.add_input_u32_slice(&to_vec(&bf_input).expect("should have input"));

    // Run prover & generate receipt
    let receipt = prover.run().expect(
        "Code should be provable unless it had an error or exceeded the maximum cycle limit",
    );

    receipt
        .verify(&MINDFUCK_ID)
        .expect("Proven code should verify");

    let journal = &receipt.journal;
    let outputs: Outputs = from_slice(&journal).expect("Journal should contain an Outputs object");

    println!("\nlike what I have no idea rly {}\n", outputs.hash); //, outputs.memory);
    let mut i = 0;
    while outputs.stdout[i] != 0 {
        print!("{}", outputs.stdout[i] as u8 as char);
        i += 1
    }

    println!("\ndone!");
}

#[cfg(test)]
mod tests {
    use mindfuck_core::Outputs;
    use mindfuck_methods::{MINDFUCK_ELF, MINDFUCK_ID};
    use risc0_zkvm::{
        serde::{from_slice, to_vec},
        Prover,
    };

    #[test]
    fn main() {
        let bf_program = include_str!("../res/hello-world.bf");
        // let bf_input = include_str!("../res/input.bin");

        // Make the prover.
        let mut prover = Prover::new(MINDFUCK_ELF)
            .expect("Prover should be constructed from matching method code & ID");
        prover.add_input_u32_slice(&to_vec(&bf_program).expect("should be serializable"));
        // prover.add_input_u32_slice(&to_vec(&bf_input).expect("should be
        // serializable"));

        // Run prover & generate receipt
        let receipt = prover.run().expect("Code should be provable");

        receipt
            .verify(&MINDFUCK_ID)
            .expect("Proven code should verify");

        let journal = &receipt.journal;
        let outputs: Outputs =
            from_slice(&journal).expect("Journal should contain an Outputs object");
        assert_eq!(
            outputs.data, 47,
            "Did not find the expected value in the critical_data field"
        );
    }
}
