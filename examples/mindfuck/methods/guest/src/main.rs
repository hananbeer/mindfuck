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

#![no_main]

// use mindfuck::parse;
// use brainfuck::{lex, parse, run};
use mindfuck_core::{lex, parse, run, Outputs};
use risc0_zkvm::{
    guest::env,
    sha::{Impl, Sha256},
};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let bf_program: String = env::read();
    // let bf_input: String = env::read();
    let sha = *Impl::hash_bytes(&bf_program.as_bytes());

    // Lex file into opcodes
    let opcodes = lex(bf_program);

    // Parse opcodes into program
    let program = parse(opcodes);

    let mut memory: Vec<u8> = vec![0; 128];
    let mut stdout: Vec<u8> = vec![0; 128];
    let mut data_pointer = 64;

    run(&program, &mut memory, &mut stdout, &mut data_pointer);

    let out = Outputs {
        memory: memory,
        stdout: stdout,
        hash: sha,
    };
    env::commit(&out);
}
