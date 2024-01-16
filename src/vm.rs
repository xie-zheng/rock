use crate::chunk::{Chunk, OpCode::*, Value};

pub enum InterpretErr {
    CompileErr,
    RuntimeErr,
}

pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
}

const STACK_MAX: usize = 256;

impl VM {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            chunk,
            ip: 0,
            stack: Vec::new(),
        }
    }

    pub fn debug(&mut self) {
        println!("stack: [{:?}]", self.stack);
        self.chunk.disassemble("test chunk");
    }

    pub fn run(&mut self) -> Result<(), InterpretErr> {
        while self.ip < self.chunk.code.len() {
            match self.chunk.code[self.ip].into() {
                CONSTANT => {
                    self.ip += 1;
                    let constant = self.chunk.constants[self.chunk.code[self.ip] as usize];
                    self.stack.push(constant);
                    println!("{constant}");
                }
                CONSTANT_LONG => {
                    self.ip += 1;
                    let mut long = (self.chunk.code[self.ip] as usize) << 16;
                    long += (self.chunk.code[self.ip + 1] as usize) << 8;
                    long += self.chunk.code[self.ip + 2] as usize;
                    let constant = self.chunk.constants_long[long];
                    self.stack.push(constant);
                    println!("{constant}");
                }
                ADD => {
                    let right = self.stack.pop().unwrap();
                    let left = self.stack.pop().unwrap();
                    self.stack.push(left + right);
                }
                SUBTRACT => {
                    let right = self.stack.pop().unwrap();
                    let left = self.stack.pop().unwrap();
                    self.stack.push(left - right);
                }
                MULTIPLY => {
                    let right = self.stack.pop().unwrap();
                    let left = self.stack.pop().unwrap();
                    self.stack.push(left * right);
                }
                DIVIDE => {
                    let right = self.stack.pop().unwrap();
                    let left = self.stack.pop().unwrap();
                    self.stack.push(left / right);
                }
                NEGATE => {
                    let x = self.stack.last_mut().unwrap();
                    *x = -*x;
                }
                RETURN => {
                    println!("{}", self.stack.pop().unwrap());
                    break;
                }
                UNKNOWN => {
                    break;
                }
            }
            self.ip += 1;
        }
        Ok(())
    }

    pub fn free(&mut self) {}
}
