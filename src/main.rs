mod chunk;
mod value;

use crate::chunk::{Chunk, OP_RETURN};

fn main() {
    let mut my_chunk = Chunk::new();

    my_chunk.write(OP_RETURN);
    my_chunk.disassemble(String::from("mah chunk"));
}
