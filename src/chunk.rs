#[repr(u8)]
pub enum OpCode {
    CONSTANT,
    RETURN,
    UNKNOWN,
}

impl From<u8> for OpCode {
    fn from(op: u8) -> Self {
        match op {
            0 => Self::CONSTANT,
            1 => Self::RETURN,
            _ => Self::UNKNOWN,
        }
    }
}

type Value = f64;

pub struct Chunk {
    code: Vec<u8>,
    /// lines的编码，lines[i]=截止到第i行的所有指令数量
    /// 这样在确定是第几行时只需二分查找一下
    pub lines: Vec<usize>,
    constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            lines: Vec::new(),
            constants: Vec::new(),
        }
    }

    fn line_add(&mut self, line: usize, count: usize) {
        if line == self.lines.len() {
            self.lines.push(0);
        }
        if line > 0 && self.lines[line] == 0 {
            self.lines[line] += self.lines[line - 1]
        }
        self.lines[line] += count;
    }

    fn get_line(&self, offset: usize) -> usize {
        let offset = offset + 1;
        match self.lines.binary_search(&offset) {
            Ok(line) => line,
            Err(line) => line,
        }
    }

    pub fn write(&mut self, op: OpCode, line: usize) {
        self.code.push(op as u8);
        self.line_add(line, 1);
    }

    pub fn write_const(&mut self, op: OpCode, data: Value, line: usize) {
        self.constants.push(data);
        self.code
            .extend_from_slice(&[op as u8, self.constants.len() as u8 - 1]);
        self.line_add(line, 2);
    }

    pub fn disassemble(&self, name: &str) {
        println!("==== {} ====", name);

        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:0>4} ", offset);
        print!("line:{:0>4} ", self.get_line(offset));

        let instruction: OpCode = self.code[offset].into();
        match instruction {
            OpCode::RETURN => Self::simple_instruction("OP_RETURN", offset),
            OpCode::CONSTANT => self.const_instruction("OP_CONSTANT", offset),
            _ => {
                println!("Unknown opcode {} \n", instruction as u8);
                offset + 1
            }
        }
    }

    fn simple_instruction(op: &str, offset: usize) -> usize {
        println!("{op}");
        offset + 1
    }

    fn const_instruction(&self, op: &str, offset: usize) -> usize {
        println!("{} {}", op, self.constants[self.code[offset + 1] as usize]);
        offset + 2
    }
}
