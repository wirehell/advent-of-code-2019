use std::env;
use crate::Instruction::{Halt, Add, Multiply, Input, Output};
use crate::Parameter::{Imm, Pos};


#[macro_use]
extern crate log;
extern crate env_logger;

type Memory = Vec<i32>;
type IP = usize;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Instruction {
    Add {op1 :Parameter, op2: Parameter, dst: Parameter},
    Multiply {op1 :Parameter, op2: Parameter, dst: Parameter},
    Input {dst :Parameter},
    Output {src :Parameter},
    Halt,
}

fn instruction_size(instruction :&Instruction) -> usize {
    return match instruction {
        Add { .. } => 4,
        Multiply { .. } => 4,
        Input { .. } => 2,
        Output { .. } => 2,
        Halt => 1,
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Parameter {
    Pos(usize),
    Imm(i32),
}

fn read_params(ip: IP, memory :&Memory, num: i32) -> Vec<Parameter> {
    let mut params: Vec<Parameter> = vec![];
    let mut param_code = memory[ip] / 100;
    for i in 1..=num {
        let val = memory[ip + i as usize];
        let p = match param_code % 10 {
            0 => Parameter::Pos(val as usize),
            1 => Parameter::Imm(val),
            _ => panic!("Invalid param: {}", memory[ip]/100),
        };
        param_code /= 10;
        params.push(p)
    }
    return params;
}

fn decode_instruction(ip: IP, memory :&Memory) -> Instruction {
    let op_code = memory[ip] % 100;
    return match op_code {
        1 => {
            let params = read_params(ip, &memory, 3);
            Add { op1: params[0], op2: params[1], dst: params[2] }
        },
        2 => {
            let params = read_params(ip, &memory, 3);
            Multiply { op1: params[0], op2: params[1], dst: params[2] }
        }
        3 => {
            let params = read_params(ip, &memory, 1);
            Input {dst: params[0]}
        }
        4 => {
            let params = read_params(ip, &memory, 1);
            Output {src: params[0]}
        }
        99 => Halt,

        _ => panic!("Unknown instruction: {:?}", memory[ip]),

    };
}

fn write(memory :&mut Memory, val: i32, dst: Parameter) {
    match dst {
        Imm(_)   => panic!("Writing IMM"),
        Pos(p) => {
            //println!("Write: mem[{}] = {}", p, val);
            memory[p] = val;
        },
    }
}

fn load(memory :&Memory, src: Parameter) -> i32 {
    match src {
        Imm(v) => {
            //println!("Load const: {}", v);
            return v;
        }
        Pos(p) => {
            let val = memory[p];
            //println!("Loaded: {} <- mem[{}]", val, p);
            return val;
        },
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Reading from file: {}", filename);
    let s = std::fs::read_to_string(filename).unwrap();
    let sl = split_and_parse(&s);
    let result = execute(sl);
}

fn run(noun:i32, verb:i32, initial:Vec<i32>) -> Vec<i32> {
    let mut state = initial;
    state[1] = noun;
    state[2] = verb;
    return execute(state);
}

fn split_and_parse(s :&str) -> Vec<i32> {
    let split = s.trim().split(",");
   return split.map(|x| x.parse::<i32>().unwrap()).collect();
}

fn execute(initial: Memory) -> Memory {
    //println!("Starting new execution");
    let mut ip = 0;
    let mut mem = initial;
    loop {
        //println!("Executing at IP:{}", ip);
        let instruction = decode_instruction(ip, &mem);
        match instruction {
            Add {op1, op2, dst} => {
                let v1 = load(&mem, op1);
                let v2 = load(&mem, op2);
                let res = v1 + v2;
                write(&mut mem, res, dst);
            },
            Multiply {op1, op2, dst} => {
                let v1 = load(&mem, op1);
                let v2 = load(&mem, op2);
                let res = v1 * v2;
                write(&mut mem, res, dst);
            },
            Input {dst} => {
                write(&mut mem, input(), dst)
            },
            Output {src} => {
                output(load(&mem, src))
            }
            Halt => {
                println!("Halt!");
                return mem;
            },
            _ => panic!("Unknown instruction: {}", ip),
        }
        ip += instruction_size(&instruction);
    }
}

fn input() -> i32 {
   return 1;
}

fn output(val :i32) {
    println!("Output: {}", val)
}

#[cfg(test)]
mod tests {
    use crate::{execute, split_and_parse, decode_instruction};
    use crate::Instruction::{Add, Multiply};
    use crate::Parameter::{Imm, Pos};

    fn init() {
        let _ = env_logger::builder()
            .is_test(true)
            .try_init();
    }

    #[test]
    fn test() {
        assert_eq!(split_and_parse(&"1,2,3"), [1,2,3])
    }

    #[test]
    fn testDecode() {
        assert_eq!(decode_instruction(0, &vec![1,0,0,0,99]),
                   Add {op1: Pos(0), op2: Pos(0), dst: Pos(0) });
        assert_eq!(decode_instruction(0, &vec![1002,4,3,4,33]),
                   Multiply {op1: Pos(4), op2: Imm(3), dst: Pos(4) });
    }

    #[test]
    fn testExecute() {
        assert_eq!(execute(vec![1,0,0,0,99]), [2,0,0,0,99]);
        assert_eq!(execute(vec![2,3,0,3,99]), [2,3,0,6,99]);
        assert_eq!(execute(vec![2,4,4,5,99,0]), [2,4,4,5,99,9801]);
        assert_eq!(execute(vec![1,1,1,4,99,5,6,0,99]), [30,1,1,4,2,5,6,0,99]);
    }

    #[test]
    fn testIO() {
//        assert_eq!(execute(vec![1,0,0,0,99]), [2,0,0,0,99]);
        execute(vec![3,0,4,0,99]);
    }

}