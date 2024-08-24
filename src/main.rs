use std::io::{self, Read};

use clap::{Args, Parser, Subcommand, ValueEnum};

use rock::{
    chunk::{Chunk, OpCode::*},
    vm::VM,
};

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    repl: bool,
    target: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    if let Some(file) = cli.target {
        run_file(file);
    }
    if cli.repl {
        repl();
    }
    let mut chunk = Chunk::new();
    // for i in 0..256 {
    chunk.write_const(1.2, 0);
    chunk.write_const(3.2, 0);
    chunk.write(NEGATE, 0);
    chunk.write_const(2.0, 0);
    chunk.write(MULTIPLY, 0);
    chunk.write(RETURN, 1);

    chunk.disassemble("test chunk");
    println!("{:?}", chunk.lines);
    let mut vm = VM::new(chunk);
    let result = vm.run();
    if result.is_ok() {
        println!("execute success");
    } else {
        println!("execute fail");
    }
}

fn run_file(file_name: String) {}

fn repl() {
    loop {
        print!("> ");

        let mut line = String::new();
        io::stdin().read_to_string(&mut line).unwrap();

        println!();

        interpret(line);
    }
}

fn interpret(line: String) {}
