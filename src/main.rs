use rock::chunk::{Chunk, OpCode::*};

fn main() {
    let mut chunk = Chunk::new();

    for i in 0..256 {
        chunk.write_const(1.2, 0);
    }
    chunk.write_const(3.2, 0);
    chunk.write(RETURN, 1);

    chunk.disassemble("test chunk");
    println!("{:?}", chunk.lines);
}
