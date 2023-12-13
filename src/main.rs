use rock::chunk::{Chunk, OpCode::*};

fn main() {
    let mut chunk = Chunk::new();

    chunk.write_const(CONSTANT, 1.2, 0);
    chunk.write(RETURN, 1);

    chunk.disassemble("test chunk");
    println!("{:?}", chunk.lines);
}
