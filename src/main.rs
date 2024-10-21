use std::{env::args, io::Read};

fn main() {
    let input_file = args().nth(1).expect("No input file given");
    let memory_size = args()
        .nth(2)
        .unwrap_or("1024".to_string())
        .parse::<usize>()
        .expect("Failed to parse memory size");
    let input_buffer = std::fs::read(input_file).expect("Failed to read input file");
    interpret(clean_input(input_buffer), create_memory(memory_size));
}

fn clean_input(buffer: Vec<u8>) -> Vec<u8> {
    buffer
        .into_iter()
        .filter(|&c| {
            c == b'>'
                || c == b'<'
                || c == b'+'
                || c == b'-'
                || c == b'.'
                || c == b','
                || c == b'['
                || c == b']'
        })
        .collect()
}

fn interpret(program: Vec<u8>, mut mem: Vec<usize>) {
    let mut pc = 0;
    let mut mp = 0;

    while pc != program.len() {
        let instruction = program[pc] as char;
        match instruction {
            '>' => {
                mp += 1;
                if mp == mem.len() {
                    panic!("Attempted to move memory pointer out of bounds (upper)");
                }
            }
            '<' => mp -= 1,
            '+' => mem[mp] += 1,
            '-' => mem[mp] -= 1,
            '.' => print!("{}", mem[mp] as u8 as char),
            ',' => {
                let mut buffer = [0; 1];
                std::io::stdin().read_exact(&mut buffer).unwrap();
                mem[mp] = buffer[0] as usize;
            }
            '[' => {
                if mem[mp] == 0 {
                    let mut balance = 1;
                    while balance != 0 {
                        pc += 1;
                        if program[pc] as char == '[' {
                            balance += 1;
                        } else if program[pc] as char == ']' {
                            balance -= 1;
                        }
                    }
                }
            }
            ']' => {
                if mem[mp] != 0 {
                    let mut balance = 1;
                    while balance != 0 {
                        pc -= 1;
                        if program[pc] as char == '[' {
                            balance -= 1;
                        } else if program[pc] as char == ']' {
                            balance += 1;
                        }
                    }
                }
            }
            _ => panic!("Unknown instruction: {}", instruction),
        }

        pc += 1;
    }
}

fn create_memory(size: usize) -> Vec<usize> {
    let mut memory = Vec::with_capacity(size);
    memory.resize(size, 0);
    memory
}
