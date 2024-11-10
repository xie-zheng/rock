use std::io::{self, Read};
use std::process::exit;

use clap::Parser;

use rock::{
    chunk::{Chunk, OpCode::*},
    vm::{InterpretErr, VM},
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

    #[cfg(debug_assertions)]
    {
        chunk.disassemble("test chunk");
        println!("{:?}", chunk.lines);
    }

    let mut vm = VM::new(chunk);
    let result = vm.run();
    if result.is_ok() {
        println!("execute success");
    } else {
        println!("execute fail");
    }
}

fn run_file(file_name: String) {
    let source = std::fs::read_to_string(file_name);
    if source.is_err() {
        exit(74);
    }
    let source_str = source.unwrap();
    let result = interpret(source_str);
    match result {
        Ok(()) => return,
        Err(InterpretErr::CompileErr) => exit(65),
        Err(InterpretErr::RuntimeErr) => exit(70),
    }
}

fn repl() {
    loop {
        print!("> ");

        let mut line = String::new();
        let result = io::stdin().read_to_string(&mut line);
        if let Err(err) = result {
            println!("interpret met err {}", err);
            return;
        }

        println!();

        match interpret(line) {
            Ok(()) => continue,
            Err(_) => break,
        }
    }
}
