use core::panic;

#[derive(Debug,Clone)]
pub enum Instruction{
    Inp(Register),
    Add{a:Register,b:Register},
    Mul{a:Register,b:Register},
    Div{a:Register,b:Register},
    Mod{a:Register,b:Register},
    Eql{a: Register, b:Register}
}
#[derive(Debug,Clone)]
pub enum Register{
    W,
    X,
    Y,
    Z,
    Num(i32)
}

#[derive(Debug)]
pub struct VM{
    instructions: Vec<Instruction>,
    w:i32,
    x:i32,
    y:i32,
    pub z:i32
}

impl VM{
    pub fn new(instructions:Vec<Instruction>) -> Self{
        VM{
            instructions,
            w:0,
            x:0,
            y:0,
            z:0
        }
    }
    pub fn run(&mut self){
        for i in 0..self.instructions.len(){
            match &self.instructions[i].clone(){
                Instruction::Inp(reg)=> {
                    let val;
                    let mut buf = String::new();
                    std::io::stdin().read_line(&mut buf).unwrap();
                    let str = buf.strip_suffix('\n').unwrap();
                    val = str.to_string().parse::<i32>().unwrap();
                    println!("Read '{}'", val);
                    self.store_val(&reg.clone(), val);
                },
                Instruction::Add{a,b} =>{
                    let val = self.get_val(a) + self.get_val(b);
                    self.store_val(a, val);
                },
                Instruction::Mul { a, b } =>{
                    let val = self.get_val(a) * self.get_val(b);
                    self.store_val(a, val);
                },
                Instruction::Div{a,b} => {
                    let val = self.get_val(a) / self.get_val(b);
                    self.store_val(a, val);
                },
                Instruction::Mod{a,b} =>{
                    let val = self.get_val(a) % self.get_val(b);
                    self.store_val(a, val);
                },
                Instruction::Eql{a,b} => {
                    let val = self.get_val(a) == self.get_val(b);
                    if val {self.store_val(a, 1)} else {self.store_val(a, 0)};
                }
            };
        }
    }

    fn store_val(&mut self, reg:&Register, val: i32){
        match reg{
            Register::W => {self.w = val},
            Register::X => {self.x = val},
            Register::Y => {self.y = val},
            Register::Z => {self.z = val},
            _ => {panic!("Failed to store reg!")}
        };
    }

    pub fn get_val(&self, reg:&Register) -> i32{
        match reg{
            Register::W => {self.w},
            Register::X => {self.x},
            Register::Y => {self.y},
            Register::Z => {self.z},
            Register::Num(x) => *x,
        }
    }

}
