use std::env;

use crate::Instruction::{Halt, Add, Multiply, Input, Output, JumpIfTrue, JumpIfFalse, LessThan, Equals};
use crate::Parameter::{Imm, Pos};

type Memory = Vec<i32>;
type OutputData = Vec<i32>;
type InputData = Vec<i32>;
type IP = usize;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Instruction {
    Add {op1 :Parameter, op2: Parameter, dst: Parameter},
    Multiply {op1 :Parameter, op2: Parameter, dst: Parameter},
    Input {dst :Parameter},
    Output {src :Parameter},
    JumpIfTrue {cond: Parameter, target: Parameter},
    JumpIfFalse {cond: Parameter, target: Parameter},
    LessThan {op1 :Parameter, op2: Parameter, dst: Parameter},
    Equals {op1 :Parameter, op2: Parameter, dst: Parameter},
    Halt,
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
        },
        3 => {
            let params = read_params(ip, &memory, 1);
            Input {dst: params[0]}
        },
        4 => {
            let params = read_params(ip, &memory, 1);
            Output {src: params[0]}
        },
        5 => {
            let params = read_params(ip, &memory, 2);
            JumpIfTrue {cond: params[0], target: params[1]}
        },
        6 => {
            let params = read_params(ip, &memory, 2);
            JumpIfFalse {cond: params[0], target: params[1]}
        },
        7 => {
            let params = read_params(ip, &memory, 3);
            LessThan {op1: params[0], op2: params[1], dst: params[2]}
        },
        8 => {
            let params = read_params(ip, &memory, 3);
            Equals {op1: params[0], op2: params[1], dst: params[2]}
        },
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
    let result = execute_with_result(sl, vec![5]);
    println!("Result: {:?}", result);
}

fn split_and_parse(s :&str) -> Vec<i32> {
    let split = s.trim().split(",");
   return split.map(|x| x.parse::<i32>().unwrap()).collect();
}

fn execute_with_result(initial: Memory, in_data: InputData) -> OutputData {
    let (_, result) =  execute(initial, in_data);
    return result;
}

fn execute(initial: Memory, in_data: Vec<i32>) -> (Memory, Vec<i32>)  {
    //println!("Starting new execution");
    let mut output = vec![];
    let mut input = in_data;

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
                ip += 4;
            },
            Multiply {op1, op2, dst} => {
                let v1 = load(&mem, op1);
                let v2 = load(&mem, op2);
                let res = v1 * v2;
                write(&mut mem, res, dst);
                ip += 4;
            },
            Input {dst} => {
                write(&mut mem, input.pop().unwrap(), dst);
                ip += 2;
            },
            Output {src} => {
                output.push(load(&mem, src));
                ip += 2;
            }
            JumpIfTrue { cond, target } => {
                let val = load(&mem, cond);
                if val != 0 {
                    let target = load(&mem, target);
                    ip = target as usize;
                } else {
                    ip += 3
                }
            }
            JumpIfFalse { cond, target } => {
                let val = load(&mem, cond);
                if val == 0 {
                    let target = load(&mem, target);
                    ip = target as usize;
                } else {
                    ip += 3
                }
            }
            LessThan { op1, op2, dst } => {
                let val1 = load(&mem, op1);
                let val2 = load(&mem, op2);
                let result;
                if val1 < val2 {
                    result = 1;
                } else {
                    result = 0;
                }
                write(&mut mem, result, dst);
                ip += 4
            }
            Equals { op1, op2, dst } => {
                let val1 = load(&mem, op1);
                let val2 = load(&mem, op2);
                let result;
                if val1 == val2 {
                    result = 1;
                } else {
                    result = 0;
                }
                write(&mut mem, result, dst);
                ip += 4
            }
            Halt => {
                println!("Halt!");
                return (mem, output);
            },
            _ => panic!("Unknown instruction: {}", ip),
        }
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
    use crate::{execute, split_and_parse, decode_instruction, execute_with_result};
    use crate::Instruction::{Add, Multiply};
    use crate::Parameter::{Imm, Pos};

    #[test]
    fn test() {
        assert_eq!(split_and_parse(&"1,2,3"), [1,2,3])
    }

    #[test]
    fn test_decode() {
        assert_eq!(decode_instruction(0, &vec![1,0,0,0,99]),
                   Add {op1: Pos(0), op2: Pos(0), dst: Pos(0) });
        assert_eq!(decode_instruction(0, &vec![1002,4,3,4,33]),
                   Multiply {op1: Pos(4), op2: Imm(3), dst: Pos(4) });
    }

    #[test]
    fn test_execute() {
        assert_eq!(execute(vec![1,0,0,0,99], vec![]), (vec![2,0,0,0,99], vec![]));
        assert_eq!(execute(vec![2,3,0,3,99], vec![]), (vec![2,3,0,6,99], vec![]));
        assert_eq!(execute(vec![2,4,4,5,99,0], vec![]), (vec![2,4,4,5,99,9801], vec![]));
        assert_eq!(execute(vec![1,1,1,4,99,5,6,0,99], vec![]), (vec![30,1,1,4,2,5,6,0,99], vec![]));
    }

    #[test]
    fn testIO() {
        assert_eq!(execute_with_result(vec![3,0,4,0,99], vec![73]), vec![73]);
    }

    #[test]
    fn test_comparision() {
        // Using position mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
        assert_eq!(execute_with_result(vec![3,9,8,9,10,9,4,9,99,-1,8], vec![8]), vec![1]);
        assert_eq!(execute_with_result(vec![3,9,8,9,10,9,4,9,99,-1,8], vec![7]), vec![0]);
        assert_eq!(execute_with_result(vec![3,9,8,9,10,9,4,9,99,-1,8], vec![9]), vec![0]);
        // Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
        assert_eq!(execute_with_result(vec![3,9,7,9,10,9,4,9,99,-1,8], vec![8]), vec![0]);
        assert_eq!(execute_with_result(vec![3,9,7,9,10,9,4,9,99,-1,8], vec![7]), vec![1]);
        assert_eq!(execute_with_result(vec![3,9,7,9,10,9,4,9,99,-1,8], vec![9]), vec![0]);
        // Using immediate mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
        assert_eq!(execute_with_result(vec![3,3,1108,-1,8,3,4,3,99], vec![7]), vec![0]);
        assert_eq!(execute_with_result(vec![3,3,1108,-1,8,3,4,3,99], vec![8]), vec![1]);
        assert_eq!(execute_with_result(vec![3,3,1108,-1,8,3,4,3,99], vec![9]), vec![0]);
        // Using immediate mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
        assert_eq!(execute_with_result(vec![3,3,1107,-1,8,3,4,3,99], vec![7]), vec![1]);
        assert_eq!(execute_with_result(vec![3,3,1107,-1,8,3,4,3,99], vec![8]), vec![0]);
        assert_eq!(execute_with_result(vec![3,3,1107,-1,8,3,4,3,99], vec![9]), vec![0]);
    }

    #[test]
    fn test_jumps() {
        // Here are some jump tests that take an input, then output 0 if the input was zero or 1 if the input was non-zero:
        assert_eq!(execute_with_result(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], vec![9]), vec![1]);
        assert_eq!(execute_with_result(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], vec![0]), vec![0]);
        assert_eq!(execute_with_result(vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], vec![9]), vec![1]);
        assert_eq!(execute_with_result(vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], vec![0]), vec![0]);
    }


    #[test]
    fn test_complex() {
        let program = vec![3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99];

        assert_eq!(execute_with_result(program.clone(), vec![8]), vec![1000]);
        assert_eq!(execute_with_result(program.clone(), vec![9]), vec![1001]);
        assert_eq!(execute_with_result(program.clone(), vec![7]), vec![999]);
    }
}