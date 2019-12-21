use std::{env, thread};
use advent_of_code_2019::intmachine;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::{HashMap, VecDeque};
use std::sync::mpsc::{SyncSender, Receiver};
use advent_of_code_2019::intmachine::{Message, Word};
use std::sync::mpsc;
use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;
use crate::State::{Input, WaitResponse};
use crate::Direction::{North, South, West, East};
use crate::Tile::{Empty, Scaffold};


#[derive(Clone, Eq, PartialEq, Debug)]
enum Tile {
    Empty,
    Scaffold,
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn from(x: Word) -> Direction {
        match x {
            1 => North,
            2 => South,
            3 => West,
            4 => East,
            _ => panic!(),

        }
    }
}


#[derive(Clone, Eq, PartialEq, Debug)]
enum State {
    WaitResponse(Direction),
    Input,
}

impl Tile {
    fn repr(&self) -> char {
        return match self {
            Empty => '.',
            Scaffold => '#',
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Robot {
    x: i32,
    y: i32,
    direction: Direction,
}


#[derive(Clone, Eq, PartialEq, Debug)]
struct Controller {
    x :i64,
    y :i64,
    map: Vec<Vec<Tile>>,
    robot: Option<Robot>,
//    state: State,
}

impl Controller {
    pub fn new() -> Controller {
        return Controller {
            x: 0,
            y: 0,
            map: vec![vec![]],
            robot: None,
        };
    }

    pub fn update(&mut self, w: &char) {
        let y = self.map.len()-1;
        match w {
            '.' => {
                self.map[y].push(Empty);
            },
            '#' =>  self.map[y].push(Scaffold),
            '^' => {},
            'v' => {},
            '>' => {},
            '<' => {},
            '\n' => self.map.push(vec![]),
            _ => {
                // panic!("Unknown symbol: {}", w);
            }


        }
    }


    pub fn print(&self) {
//        print!("{}[2J", 27 as char);
//        print!("\x1B[2J");

        for y in self.map.iter() {
            for x in y {
                print!("{}", x.repr());

            }
            println!("");
        }
    }

}

fn main() {
    let filename = "data/day21/input.txt";
    let mut program = intmachine::read_program(filename);

//    let mut controller:Controller = Controller::new();

    let (input, pin): (SyncSender<Message>, Receiver<Message>) = mpsc::sync_channel(0);
    let (pout, output): (SyncSender<Message>, Receiver<Message>) = mpsc::sync_channel(0);

    let child = thread::spawn(move || {
        intmachine::execute(&program, pin, pout);
    });

    let mut rng = rand::thread_rng();

    let mut count = 0;
    /*
    let instructions = vec![
        "NOT A J",
        "NOT B T",
        "OR T J",
        "NOT C T",
        "OR T J",
        "AND D J",
        "WALK",
    ];
    */
    let instructions = vec![
        // Jump if A,B or C
        "NOT A J",
        "NOT B T",
        "OR T J",
        "NOT C T",
        "OR T J",

        // Don't jump to a hole
        "AND D J",

        // Don't jump to a place where there is nothing after, and must jump to hole
        "NOT E T",
        "NOT T T",
        "OR H T",
        "AND T J",

        "RUN",
    ];

    let mut d: VecDeque<char> = VecDeque::new();
    for line in instructions {
        d.extend(line.chars());
        d.push_back('\n');
    }



    loop {

        match output.recv() {
            Ok(message) => {
                match message {
                    Message::Data(data) =>  {
                        if data > 256 {
                            println!("Received: {}", data);
                        }
                        let c = char::from(data as u8);
//                        controller.update(&c);
                        print!("{}", c);
                    }

                    Message::Shutdown => {
                        println!("Shutdown..");
                        break;
                    }
                    Message::RequestInput => {
//                        println!("d: {:?}",d);
                        let mc = d.pop_front();

                        let c :char = mc.unwrap();

//                        println!("sending: {}", &c);
                        input.send(Message::Data(c as Word));

//                        println!("Random walk: {}", &r);
//                        input.send(Message::Data(r));
//                        controller.state = WaitResponse(Direction::from(r));
                    }
                }
            }
            Err(error) => {
                panic!("Error: {:?}", error.to_string());
            }
        }
    }
    child.join();

    //controller.print();
//    println!("Score: {}", controller.align());

}

#[cfg(test)]
mod tests {

}
