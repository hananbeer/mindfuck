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

use risc0_zkvm::sha::Digest;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Outputs {
    pub memory: Vec<u8>,
    pub stdout: Vec<u8>,
    // pub data: u32,
    pub hash: Digest,
}

/// Opcodes determined by the lexer
#[derive(Debug, Clone)]
pub enum OpCode {
    IncrementPointer,
    DecrementPointer,
    Increment,
    Decrement,
    Write,
    Read,
    LoopBegin,
    LoopEnd,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    IncrementPointer,
    DecrementPointer,
    Increment,
    Decrement,
    Write,
    Read,
    Loop(Vec<Instruction>),
}

/// Lexer turns the source code into a sequence of opcodes
pub fn lex(source: String) -> Vec<OpCode> {
    let mut operations = Vec::new();

    for symbol in source.chars() {
        let op = match symbol {
            '>' => Some(OpCode::IncrementPointer),
            '<' => Some(OpCode::DecrementPointer),
            '+' => Some(OpCode::Increment),
            '-' => Some(OpCode::Decrement),
            '.' => Some(OpCode::Write),
            ',' => Some(OpCode::Read),
            '[' => Some(OpCode::LoopBegin),
            ']' => Some(OpCode::LoopEnd),
            _ => None,
        };

        // Non-opcode characters are simply comments
        match op {
            Some(op) => operations.push(op),
            None => (),
        }
    }

    operations
}

pub fn parse(opcodes: Vec<OpCode>) -> Vec<Instruction> {
    let mut program: Vec<Instruction> = Vec::new();
    let mut loop_stack = 0;
    let mut loop_start = 0;

    for (i, op) in opcodes.iter().enumerate() {
        if loop_stack == 0 {
            let instr = match op {
                OpCode::IncrementPointer => Some(Instruction::IncrementPointer),
                OpCode::DecrementPointer => Some(Instruction::DecrementPointer),
                OpCode::Increment => Some(Instruction::Increment),
                OpCode::Decrement => Some(Instruction::Decrement),
                OpCode::Write => Some(Instruction::Write),
                OpCode::Read => Some(Instruction::Read),

                OpCode::LoopBegin => {
                    loop_start = i;
                    loop_stack += 1;
                    None
                }

                OpCode::LoopEnd => panic!("loop ending at #{} has no beginning", i),
            };

            match instr {
                Some(instr) => program.push(instr),
                None => (),
            }
        } else {
            match op {
                OpCode::LoopBegin => {
                    loop_stack += 1;
                }
                OpCode::LoopEnd => {
                    loop_stack -= 1;

                    if loop_stack == 0 {
                        program.push(Instruction::Loop(parse(
                            opcodes[loop_start + 1..i].to_vec(),
                        )));
                    }
                }
                _ => (),
            }
        }
    }

    if loop_stack != 0 {
        panic!(
            "loop that starts at #{} has no matching ending!",
            loop_start
        );
    }

    program
}

/// Executes a program that was previously parsed
pub fn run(
    instructions: &Vec<Instruction>,
    memory: &mut Vec<u8>,
    stdout: &mut Vec<u8>,
    data_pointer: &mut usize,
) {
    let mut out_offset = 0;
    for instr in instructions {
        match instr {
            Instruction::IncrementPointer => *data_pointer += 1,
            Instruction::DecrementPointer => *data_pointer -= 1,
            Instruction::Increment => memory[*data_pointer] += 1,
            Instruction::Decrement => memory[*data_pointer] -= 1,
            Instruction::Write => {
                // print!("{}", memory[*data_pointer] as char)
                stdout[out_offset] = memory[*data_pointer];
                out_offset += 1
            }
            Instruction::Read => {
                // let mut input: [u8; 1] = [0; 1];
                // std::io::stdin()
                //     .read_exact(&mut input)
                //     .expect("failed to read stdin");
                // memory[*data_pointer] = input[0];
            }
            Instruction::Loop(nested_instructions) => {
                while memory[*data_pointer] != 0 {
                    run(&nested_instructions, memory, stdout, data_pointer)
                }
            }
        }
    }
}
