use core::panic;

#[derive(Debug, Clone)]
pub enum Instruction {
    Inp(Register),
    Add { a: Register, b: Register },
    Mul { a: Register, b: Register },
    Div { a: Register, b: Register },
    Mod { a: Register, b: Register },
    Eql { a: Register, b: Register },
}
#[derive(Debug, Clone)]
pub enum Register {
    W,
    X,
    Y,
    Z,
    Num(i64),
}

#[derive(Debug)]
pub struct VM {
    instructions: Vec<Instruction>,
    w: i64,
    x: i64,
    y: i64,
    pub z: i64,
}

impl VM {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        VM {
            instructions,
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }
    pub fn run(&mut self, input: Option<&[u32]>) {
        let mut count = 0;
        for i in 0..self.instructions.len() {
            match &self.instructions[i].clone() {
                Instruction::Inp(reg) => {
                    let val:i64;
                    if let Some(nums) = input {
                        val = nums[count] as i64;
                        count += 1;
                    } else {
                        let mut buf = String::new();
                        std::io::stdin().read_line(&mut buf).unwrap();
                        let str = buf.strip_suffix('\n').unwrap();
                        val = str.to_string().parse::<i64>().unwrap();
                    }
                    self.store_val(&reg.clone(), val);
                }
                Instruction::Add { a, b } => {
                    let val = self.get_val(a) + self.get_val(b);
                    self.store_val(a, val);
                }
                Instruction::Mul { a, b } => {
                    let val = self.get_val(a) * self.get_val(b);
                    self.store_val(a, val);
                }
                Instruction::Div { a, b } => {
                    let val = self.get_val(a) / self.get_val(b);
                    self.store_val(a, val);
                }
                Instruction::Mod { a, b } => {
                    let val = self.get_val(a) % self.get_val(b);
                    self.store_val(a, val);
                }
                Instruction::Eql { a, b } => {
                    let val = self.get_val(a) == self.get_val(b);
                    if val {
                        self.store_val(a, 1)
                    } else {
                        self.store_val(a, 0)
                    };
                }
            };
        }
    }

    fn store_val(&mut self, reg: &Register, val: i64) {
        match reg {
            Register::W => self.w = val,
            Register::X => self.x = val,
            Register::Y => self.y = val,
            Register::Z => self.z = val,
            _ => {
                panic!("Failed to store reg!")
            }
        };
    }

    pub fn get_val(&self, reg: &Register) -> i64 {
        match reg {
            Register::W => self.w,
            Register::X => self.x,
            Register::Y => self.y,
            Register::Z => self.z,
            Register::Num(x) => *x,
        }
    }

    pub fn reset(&mut self){
        self.w = 0;
        self.x = 0;
        self.y = 0;
        self.z = 0;
    }
}
