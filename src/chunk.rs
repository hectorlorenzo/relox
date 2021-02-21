use crate::value::{ValuesArray};

pub const OP_RETURN: u8 = 0;

pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: ValuesArray,
}

fn simple_instruction(name: String, offset: usize) -> usize {
    println!("{:04} {}", offset, name);
    return offset + 1;
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::new(),
            constants: ValuesArray::new(),
        }
    }

    pub fn write(&mut self, byte: u8) {
        self.code.push(byte);
    }

    pub fn disassemble(&self, header: String) {
        let mut offset = 0;
        let code_length = self.code.len();

        println!("== {} ==", header);

        while offset < code_length {
            let instruction = &self.code[offset];

            offset = match offset as u8 {
                OP_RETURN => simple_instruction(String::from("OP_RETURN"), offset),
                _ => {
                    println!("Unknown opcode {}\n", instruction);
                    offset + 1
                }
            };
        }
    }
}