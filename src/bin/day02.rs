use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Reading from file: {}", filename);
    let s = std::fs::read_to_string(filename).unwrap();

    let sl = split_and_parse(&s);
    let result = find(19690720, sl);
    println!("Result: {}", result);

}

fn find(result:i32, initial:Vec<i32>) -> i32 {
    for noun in 0..255 {
        for verb in 0..173 {
            let r = run(noun, verb, initial.clone());
            if r[0] == result {
                return 100*noun + verb;
            }
        }
    }
    panic!("Not found");
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

fn execute(initial: Vec<i32>) -> Vec<i32> {
    let mut ip = 0;
    let mut state = initial;
    loop {
        let op_code = state[ip as usize];
        if op_code == 99 {
            break;
        }
        match op_code {
            1 => {
                // dst = op1 + op2
                let op1 = state[ip + 1];
                let op2 = state[ip + 2];
                let dst = state[ip + 3];
                state[dst as usize] = state[op1 as usize] + state[op2 as usize];
                ip += 4;
            }
            2 => {
                // dst = op1 * op2
                let op1 = state[ip + 1];
                let op2 = state[ip + 2];
                let dst = state[ip + 3];
                state[dst as usize] = state[op1 as usize]* state[op2 as usize];
                ip += 4;
            }
            _ => panic!(),

        }
    }
    return state;
}


#[cfg(test)]
mod tests {
    use crate::{find, execute, split_and_parse};

    #[test]
    fn test() {
        assert_eq!(split_and_parse(&"1,2,3"), [1,2,3])
    }

    #[test]
    fn testExecute() {
        assert_eq!(execute(vec![1,0,0,0,99]), [2,0,0,0,99]);
        assert_eq!(execute(vec![2,3,0,3,99]), [2,3,0,6,99]);
        assert_eq!(execute(vec![2,4,4,5,99,0]), [2,4,4,5,99,9801]);
        assert_eq!(execute(vec![1,1,1,4,99,5,6,0,99]), [30,1,1,4,2,5,6,0,99]);
    }

    #[test]
    fn testFind() {
        assert_eq!(find(10566835, vec![1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,13,1,19,1,6,19,23,2,6,23,27,1,5,27,31,2,31,9,35,1,35,5,39,1,39,5,43,1,43,10,47,2,6,47,51,1,51,5,55,2,55,6,59,1,5,59,63,2,63,6,67,1,5,67,71,1,71,6,75,2,75,10,79,1,79,5,83,2,83,6,87,1,87,5,91,2,9,91,95,1,95,6,99,2,9,99,103,2,9,103,107,1,5,107,111,1,111,5,115,1,115,13,119,1,13,119,123,2,6,123,127,1,5,127,131,1,9,131,135,1,135,9,139,2,139,6,143,1,143,5,147,2,147,6,151,1,5,151,155,2,6,155,159,1,159,2,163,1,9,163,0,99,2,0,14,0
]), 1202)
    }

}