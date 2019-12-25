use Instruction::{Halt, Add, Multiply, Input, Output, JumpIfTrue, JumpIfFalse, LessThan, Equals, AdjustRelativeBase};
use Parameter::{Imm, Pos, Rel};
use std::collections::VecDeque;
use std::sync::mpsc::{Receiver, SyncSender};
use std::sync::mpsc;
use std::thread;
use std::io::{Write, stdout};
use std::thread::sleep;
use std::time::Duration;
use crate::intmachine::Message::RequestInput;

pub type Word = i64;
pub type Memory = Vec<Word>;
type OutputData = Vec<Word>;

#[derive(Debug)]
pub enum Message {
    Shutdown,
    Data(Word),
    RequestInput,
}


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct ProcessorState {
    ip :Word,
    relative_base: Word,
}

pub trait IO {
    fn send(&mut self, message: Message) -> ();
    fn receive(&mut self) -> Message;
}

pub struct StandardIO {
    pub stdin: Receiver<Message>,
    pub stdout: SyncSender<Message>,
}

impl IO for StandardIO {
    fn send(&mut self, message: Message) {
        self.stdout.send(message).unwrap();
    }

    fn receive(&mut self) -> Message {
        return self.stdin.recv().unwrap();
    }
}

struct IntMachine {
    memory: Memory,
    state: ProcessorState,
    io: dyn IO,
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
    AdjustRelativeBase {op :Parameter},
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Parameter {
    Pos(Word),
    Imm(Word),
    Rel(Word),
}

fn read_params(ip: &Word, memory :&Memory, num: Word) -> Vec<Parameter> {
    let mut params: Vec<Parameter> = vec![];
    let mut param_code = memory[*ip as usize] / 100;
    for i in 1..=num {
        let val = memory[(ip + i) as usize];
        let p = match param_code % 10 {
            0 => Parameter::Pos(val),
            1 => Parameter::Imm(val),
            2 => Parameter::Rel(val),
            _ => panic!("Invalid param: {}", memory[*ip as usize]/100),
        };
        param_code /= 10;
        params.push(p)
    }
    return params;
}

fn decode_instruction(ip: &Word, memory :&Memory) -> Instruction {
    let op_code = memory[*ip as usize] % 100;
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
            Input { dst: params[0] }
        },
        4 => {
            let params = read_params(ip, &memory, 1);
            Output { src: params[0] }
        },
        5 => {
            let params = read_params(ip, &memory, 2);
            JumpIfTrue { cond: params[0], target: params[1] }
        },
        6 => {
            let params = read_params(ip, &memory, 2);
            JumpIfFalse { cond: params[0], target: params[1] }
        },
        7 => {
            let params = read_params(ip, &memory, 3);
            LessThan { op1: params[0], op2: params[1], dst: params[2] }
        },
        8 => {
            let params = read_params(ip, &memory, 3);
            Equals { op1: params[0], op2: params[1], dst: params[2] }
        },
        9 => {
            let params = read_params(ip, &memory, 1);
            AdjustRelativeBase { op: params[0] }
        },
        99 => Halt,
        _ => {
            panic!("Unknown instruction: {:?}", memory[*ip as usize]);
        },
    };
}

fn write_raw(memory :&mut Memory, address: &Word, val: Word) {
    let a = *address as usize;
    if a >= memory.len() {
        println!("Out of memory write");
        panic!("Writing out of memory");
    }
    memory[a] = val;
}

fn write(mut memory :&mut Memory, state: &ProcessorState, val: Word, dst: Parameter) {
    match dst {
        Imm(_)   => panic!("Writing IMM"),
        Pos(p) => {
            write_raw(&mut memory, &p, val);
        },
        Rel(p) => {
            let address = state.relative_base + p;
            write_raw(&mut memory, &address, val);
        }
    }
}

fn load_raw(memory: &Memory, address :&Word) -> Word {
    let a = *address as usize;
    if a >= memory.len() {
        println!("Out of memory load");
        panic!("Loading out of memory");
    }
    let val = memory[a];
    //println!("Loaded: {} <- mem[{}]", val, p);
    return val;
}

fn load(memory :&Memory, state: &ProcessorState, src: Parameter) -> Word {
    match src {
        Imm(v) => {
            //println!("Load const: {}", v);
            return v;
        }
        Pos(p) => {
            return load_raw(memory, &p);
        },
        Rel(offset) => {
            let address = state.relative_base + offset;
            return load_raw(&memory, &address);
        }
    }
}

pub fn read_program(filename :&str) -> Memory {
    println!("Reading from file: {}", filename);
    let s = std::fs::read_to_string(filename).unwrap();
    let sl = split_and_parse(&s);
    return sl;
}

fn split_and_parse(s :&str) -> Vec<Word> {
    let split = s.trim().split(",");
   return split.map(|x| x.parse::<Word>().unwrap()).collect();
}

pub fn execute_with_result(initial: &Memory, in_data: Vec<Word>) -> OutputData {
    let (input, pin): (SyncSender<Message>, Receiver<Message>) = mpsc::sync_channel(0);
    let (pout, output): (SyncSender<Message>, Receiver<Message>) = mpsc::sync_channel(0);
    let mem = initial.clone();

    let child = thread::spawn(move || {
        let mut io = StandardIO {
            stdin: pin,
            stdout: pout,
        };
        execute(&mem, &mut io);
    });
    let mut inp = VecDeque::from(in_data);

    let mut result = vec![];
    loop {
        match output.recv() {
            Ok(message) => {
                match message {
                    Message::Data(data) => result.push(data),
                    Message::RequestInput => {
                        let data = inp.pop_front().unwrap();
                        input.send(Message::Data(data)).unwrap();
                    },
                    Message::Shutdown => break,
                }
            }
            Err(error) => {
                panic!("Error: {:?}", error.to_string());
            }
        }
    }
    child.join().unwrap_err();
    return result;
}

/*
pub fn execute(initial: &Memory, network_receive: Receiver<Message>, network_send: SyncSender<Message>) {



}
*/

pub fn execute(initial: &Memory, io: &mut dyn IO) -> (Memory) {

    const LEN :i64 = 640 * 1024 * 1024; // Should be enough..
    let mut mem = vec![0; LEN as usize];
    for i in 0..initial.len() {
        mem[i] = initial[i];
    }
    let mut state = ProcessorState { ip: 0, relative_base: 0 };

    loop {
        if execute_step(&mut mem, &mut state, io) {
            break;
        }
    }
    return mem;

}
fn execute_step(mut mem: &mut Memory, mut state: &mut ProcessorState, io: &mut dyn IO) -> bool{

    let instruction = decode_instruction(&state.ip, &mem);
//    println!("Executing: {:?} {:?}", state, instruction);
 //   stdout().flush();
//    sleep(Duration::from_millis(100));
    match instruction {
        Add {op1, op2, dst} => {
            let v1 = load(&mem, &state, op1);
            let v2 = load(&mem, &state, op2);
            let res = v1 + v2;
            write(&mut mem, &state, res, dst);
            state.ip += 4;
        },
        Multiply {op1, op2, dst} => {
            let v1 = load(&mem, &state, op1);
            let v2 = load(&mem, &state, op2);
            let res = v1 * v2;
            write(&mut mem, &state, res, dst);
            state.ip += 4;
        },
        Input {dst} => {
            io.send(RequestInput);
            let val = match io.receive() {
                Message::Data(data) => data,
                x => panic!(x),
            };
            write(&mut mem, &state, val, dst);
            state.ip += 2;
        },
        Output {src} => {
            let val = load(&mem, &state, src);
//            println!("Output: {}", val);
            io.send(Message::Data(val));
            state.ip += 2;
        }
        JumpIfTrue { cond, target } => {
            let val = load(&mem, &state, cond);
            if val != 0 {
                let target = load(&mem, &state, target);
                state.ip = target;
            } else {
                state.ip += 3
            }
        }
        JumpIfFalse { cond, target } => {
            let val = load(&mem, &state, cond);
            if val == 0 {
                let target = load(&mem, &state, target);
                state.ip = target;
            } else {
                state.ip += 3
            }
        }
        LessThan { op1, op2, dst } => {
            let val1 = load(&mem, &state, op1);
            let val2 = load(&mem, &state, op2);
            let result;
            if val1 < val2 {
                result = 1;
            } else {
                result = 0;
            }
            write(&mut mem, &state, result, dst);
            state.ip += 4
        }
        Equals { op1, op2, dst } => {
            let val1 = load(&mem, &state, op1);
            let val2 = load(&mem, &state, op2);
            let result;
//            print!("Comparing: {} and {}", val1, val2);
            if val1 == val2 {
                result = 1;
            } else {
                result = 0;
            }
            write(&mut mem, &state, result, dst);
            state.ip += 4
        }
        Halt => {
            io.send(Message::Shutdown);
//            println!("Halt!");
            return true;
        },
        AdjustRelativeBase { op } => {
            //println!("Adjusting!");
            state.relative_base += load(&mut mem, &state, op);
            state.ip += 2;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use crate::intmachine::{split_and_parse, decode_instruction, execute_with_result};
    use crate::intmachine::Instruction::{Add, Multiply};
    use crate::intmachine::Parameter::{Pos, Imm};

    #[test]
    fn test() {
        assert_eq!(split_and_parse(&"1,2,3"), [1,2,3])
    }

    #[test]
    fn test_decode() {
        assert_eq!(decode_instruction(&0, &vec![1,0,0,0,99]),
                   Add {op1: Pos(0), op2: Pos(0), dst: Pos(0) });
        assert_eq!(decode_instruction(&0, &vec![1002,4,3,4,33]),
                   Multiply {op1: Pos(4), op2: Imm(3), dst: Pos(4) });
    }

    #[test]
    fn testIO() {
        assert_eq!(execute_with_result(&vec![3,0,4,0,99], vec![73]), vec![73]);
    }

    #[test]
    fn test_comparision() {
        // Using position mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
        assert_eq!(execute_with_result(&vec![3,9,8,9,10,9,4,9,99,-1,8], vec![8]), vec![1]);
        assert_eq!(execute_with_result(&vec![3,9,8,9,10,9,4,9,99,-1,8], vec![7]), vec![0]);
        assert_eq!(execute_with_result(&vec![3,9,8,9,10,9,4,9,99,-1,8], vec![9]), vec![0]);
        // Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
        assert_eq!(execute_with_result(&vec![3,9,7,9,10,9,4,9,99,-1,8], vec![8]), vec![0]);
        assert_eq!(execute_with_result(&vec![3,9,7,9,10,9,4,9,99,-1,8], vec![7]), vec![1]);
        assert_eq!(execute_with_result(&vec![3,9,7,9,10,9,4,9,99,-1,8], vec![9]), vec![0]);
        // Using immediate mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
        assert_eq!(execute_with_result(&vec![3,3,1108,-1,8,3,4,3,99], vec![7]), vec![0]);
        assert_eq!(execute_with_result(&vec![3,3,1108,-1,8,3,4,3,99], vec![8]), vec![1]);
        assert_eq!(execute_with_result(&vec![3,3,1108,-1,8,3,4,3,99], vec![9]), vec![0]);
        // Using immediate mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
        assert_eq!(execute_with_result(&vec![3,3,1107,-1,8,3,4,3,99], vec![7]), vec![1]);
        assert_eq!(execute_with_result(&vec![3,3,1107,-1,8,3,4,3,99], vec![8]), vec![0]);
        assert_eq!(execute_with_result(&vec![3,3,1107,-1,8,3,4,3,99], vec![9]), vec![0]);
    }

    #[test]
    fn test_jumps() {
        // Here are some jump tests that take an input, then output 0 if the input was zero or 1 if the input was non-zero:
        assert_eq!(execute_with_result(&vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], vec![9]), vec![1]);
        assert_eq!(execute_with_result(&vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], vec![0]), vec![0]);
        assert_eq!(execute_with_result(&vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], vec![9]), vec![1]);
        assert_eq!(execute_with_result(&vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], vec![0]), vec![0]);
    }


    #[test]
    fn test_complex() {
        let program = vec![3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99];

        assert_eq!(execute_with_result(&program, vec![8]), vec![1000]);
        assert_eq!(execute_with_result(&program, vec![9]), vec![1001]);
        assert_eq!(execute_with_result(&program, vec![7]), vec![999]);
    }

    #[test]
    fn test_indirect() {
        let program = vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99];
        let expected = program.clone();
        assert_eq!(execute_with_result(&program, vec![]), expected);
    }

    #[test]
    fn test_large_number() {
        let program = vec![1102,34915192,34915192,7,4,7,99,0];
        let result = execute_with_result(&program, vec![]);
        let s = result[0].to_string();
        assert_eq!(s.len(), 16);
    }
    #[test]
    fn test_large_number2() {
        let program2 = vec![104,1125899906842624,99];
        assert_eq!(execute_with_result(&program2, vec![]), vec![1125899906842624]);
    }
}