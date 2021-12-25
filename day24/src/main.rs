use std::{io::{BufRead, BufReader}, fs::File};

mod alu;
fn main() {
    let data = read_data("data.txt");
    let mut alu = alu::VM::new(data);
    alu.run();
    println!("{:?}", alu.z);
}


fn read_data(filename:&str) -> Vec<alu::Instruction>{
    let mut result = Vec::new();
    let reader = BufReader::new(File::open(filename).unwrap());
    for line in reader.lines(){
        let unwrapped = line.unwrap();
        let split:Vec<&str> = unwrapped.split_whitespace().collect();
        //First 3 are instruction next is reg, last is variable
        match split[0]{
            "inp" => {
                let reg = str_to_reg(split[1]);
                result.push(alu::Instruction::Inp(reg));
            },
            "add" => {
                let reg1 = str_to_reg(split[1]);
                let reg2 = str_to_reg(split[2]);
                result.push(alu::Instruction::Add{
                    a:reg1,
                    b:reg2
                });
            },
            "mul" => {
                let reg1 = str_to_reg(split[1]);
                let reg2 = str_to_reg(split[2]);
                result.push(alu::Instruction::Mul{
                    a:reg1,
                    b:reg2
                });
            },
            "div" => {
                let reg1 = str_to_reg(split[1]);
                let reg2 = str_to_reg(split[2]);
                result.push(alu::Instruction::Div{
                    a:reg1,
                    b:reg2
                });
                
            },
            "mod" =>{
                let reg1 = str_to_reg(split[1]);
                let reg2 = str_to_reg(split[2]);
                result.push(alu::Instruction::Mod{
                    a:reg1,
                    b:reg2
                });

            },
            "eql" =>{
                let reg1 = str_to_reg(split[1]);
                let reg2 = str_to_reg(split[2]);
                result.push(alu::Instruction::Eql{
                    a:reg1,
                    b:reg2
                });

            }
            _ => {panic!("Invalid command")}
        }
    }

    result
}

fn str_to_reg(reg :&str) -> alu::Register{
    match reg{
        "w" => {alu::Register::W},
        "x" =>{alu::Register::X},
        "y"=>{alu::Register::Y},
        "z"=>{alu::Register::Z},
        x => {alu::Register::Num(x.parse().unwrap())}
    }
}