use std::env;

use crate::Instruction::{Halt, Add, Multiply, Input, Output, JumpIfTrue, JumpIfFalse, LessThan, Equals};
use crate::Parameter::{Imm, Pos};
use std::collections::VecDeque;
use std::sync::mpsc::{Receiver, SyncSender};
use std::sync::mpsc;
use std::thread;

type Memory = Vec<i32>;
type OutputData = Vec<i32>;
type InputData = VecDeque<i32>;
type IP = usize;

enum Message {
    Shutdown,
    Data(i32),
}

pub fn permutations(size: usize) -> Permutations {
    Permutations { idxs: (0..size).collect(), swaps: vec![0; size], i: 0 }
}

pub struct Permutations {
    idxs: Vec<usize>,
    swaps: Vec<usize>,
    i: usize,
}

impl Iterator for Permutations {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i > 0 {
            loop {
                if self.i >= self.swaps.len() { return None; }
                if self.swaps[self.i] < self.i { break; }
                self.swaps[self.i] = 0;
                self.i += 1;
            }
            self.idxs.swap(self.i, (self.i & 1) * self.swaps[self.i]);
            self.swaps[self.i] += 1;
        }
        self.i = 1;
        Some(self.idxs.clone())
    }
}

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

    let result = find_max_feedback(sl);
    println!("Result: {:?}", result);
}

fn split_and_parse(s :&str) -> Vec<i32> {
    let split = s.trim().split(",");
   return split.map(|x| x.parse::<i32>().unwrap()).collect();
}

fn execute_with_result(initial: Memory, in_data: Vec<i32>) -> OutputData {
    let (input, pin): (SyncSender<Message>, Receiver<Message>) = mpsc::sync_channel(0);
    let (pout, output): (SyncSender<Message>, Receiver<Message>) = mpsc::sync_channel(0);

    let child = thread::spawn(move || {
        execute(initial, pin, pout);
    });

    for data in in_data {
        input.send(Message::Data(data));
    }

    let mut result = vec![];
    loop {
        match output.recv().unwrap() {
            Message::Data(data) => result.push(data),
            Message::Shutdown => break,
        }
    }
    child.join();
    return result;
}

fn execute_feedback(initial: Memory, setting: Vec<i32>) -> i32 {
    let (input_writer, input_reader): (SyncSender<Message>, Receiver<Message>) = mpsc::sync_channel(1);

    let mut step_reader = input_reader;
    let mut step_input_writer = input_writer.clone();

    let mut children = Vec::new();
    for s in setting {
        let (s_out_writer, next_in_reader) : (SyncSender<Message>, Receiver<Message>) = mpsc::sync_channel(1);
        // Send setting
        step_input_writer.send(Message::Data(s));
        let child_writer = s_out_writer.clone();
        let mem = initial.clone();
        let child = thread::spawn(move || {
            execute(mem, step_reader, child_writer);
        });
        children.push(child);
        step_reader = next_in_reader;
        step_input_writer = s_out_writer;
    }
    let output = step_reader;

    // Init
    input_writer.send(Message::Data(0));

    let mut max = i32::min_value();
    loop {
        match output.recv().unwrap() {
            Message::Data(data) => {
                if data > max {
                    max = data;
                }
                input_writer.send(Message::Data(data));
            },
            Message::Shutdown => break,
        }
    }
    for child in children {
        child.join();
    }

    return max;
}

fn execute_phaser(initial: Memory, setting: Vec<i32>) -> i32 {
    let mut result = 0;
    for setting in setting {
        let output = execute_with_result(initial.clone(), vec![setting, result]);
        assert_eq!(output.len(), 1);
        result = output[0];
    }
    return result;
}

fn find_max_phaser(initial: Memory) -> i32 {
    let code = &[0,1,2,3,4];
    let mut max = -9999999;

    for perm in permutations(5) {
        let settings :Vec<i32> = vec![
            code[perm[0]],
            code[perm[1]],
            code[perm[2]],
            code[perm[3]],
            code[perm[4]],
        ];
//        println!("Executing with setting: {:?}", settings);
        let result = execute_phaser(initial.clone(),settings);
//        println!("Result: {}", result);
        if result > max {
            max = result;
        }
    }

    return max;
}

fn find_max_feedback(initial: Memory) -> i32 {
    let code = &[5,6,7,8,9];
    let mut max = -9999999;

    for perm in permutations(5) {
        let settings :Vec<i32> = vec![
            code[perm[0]],
            code[perm[1]],
            code[perm[2]],
            code[perm[3]],
            code[perm[4]],
        ];
//        println!("Executing with setting: {:?}", settings);
        let result = execute_feedback(initial.clone(),settings);
//        println!("Result: {}", result);
        if result > max {
            max = result;
        }
    }

    return max;
}

fn execute(initial: Memory, stdin: Receiver<Message>, stdout: SyncSender<Message>) -> (Memory)  {
    let mut ip = 0;
    let mut mem = initial;
    loop {
        //println!("Executing at IP:{}", ip);
        let instruction = decode_instruction(ip, &mem);
//        println!("Executing: {:?}", instruction);
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
                let val = match stdin.recv() {
                    Ok(Message::Data(data)) => data,
                    x => panic!(x),
                };
                write(&mut mem, val, dst);
                ip += 2;
            },
            Output {src} => {
                let val = load(&mem, src);
                stdout.send(Message::Data(val));
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
                stdout.send(Message::Shutdown);
            //    println!("Halt!");
                return mem;
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{split_and_parse, decode_instruction, execute_with_result, find_max_phaser, find_max_feedback};
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

    #[test]
    fn test_phaser() {
        assert_eq!(find_max_phaser(vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0]),
                   43210);
        assert_eq!(find_max_phaser(vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0]),
                   54321);
        assert_eq!(find_max_phaser(vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0]),
                   65210);
    }

    #[test]
    fn test_feedback() {
        assert_eq!(find_max_feedback(vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5]),
                   139629729);
        assert_eq!(find_max_feedback(vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10]),
                   18216);

    }
}