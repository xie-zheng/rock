use rock::chunk::{Chunk, OpCode::*};

fn main() {
    let mut chunk = Chunk::new();

    chunk.write_const(CONSTANT, 1.2);
    chunk.write(RETURN);

    chunk.disassemble("test chunk");
}
