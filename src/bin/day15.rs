use std::{env, thread};
use advent_of_code_2019::intmachine;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
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
use crate::Tile::{Empty, Unknown, Wall, OxygenSystem, Start};


#[derive(Clone, Eq, PartialEq, Debug)]
enum Tile {
    Unknown,
    Start,
    Empty,
    Wall,
    OxygenSystem,
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
            Wall => '#',
            Start => 'O',
            OxygenSystem => '%',
            Unknown => ' ',
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Controller {
    x_size :i64,
    y_size :i64,
    map: Vec<Tile>,
    dist: Vec<i64>,
    robot_pos: (Word, Word),
    state: State,
}

impl Controller {
    pub fn new(x_size :i64, y_size :i64) -> Controller {
        let ms = (x_size * y_size) as usize;
        let start = (x_size/2, y_size/2);
        let mut c = Controller {
            x_size,
            y_size,
            map: vec![Unknown; ms],
            dist: vec![999999999 ; ms],
            robot_pos: start,
            state: Input,
        };
        c.update_tile(&start.0, &start.1, Start);
        //c.set_dist(start.0, start.1, 0);
        return c;
    }

    pub fn update_tile(&mut self, x: &i64, y: &i64, tile: Tile) {
        self.map[(y * self.x_size + x) as usize] = tile;
    }

    pub fn get_dist(&self, x: i64, y: i64) -> i64 {
        return self.dist[(y * self.x_size + x) as usize];
    }
    pub fn set_dist(&mut self, x: i64, y: i64, dist: i64) {
        self.dist[(y * self.x_size + x) as usize] = dist;
    }
    pub fn update_dist(&mut self, x: i64, y: i64) -> i64 {
        let up = self.get_dist(x, y-1);
        let down = self.get_dist(x, y+1);
        let west = self.get_dist(x-1, y);
        let east = self.get_dist(x+1, y);
        let s = self.get_dist(x,y);
        let m = i64::min(i64::min(up, down), i64::min(west, east)) + 1;
        if s > m {
            self.set_dist(x,y,m);
            return m;
        } else {
            return s;
        }
    }

    pub fn get_max_dist(&self) -> i64 {
        let mut max = 0;
        for x in self.dist.iter() {
            if *x < 9999999  && *x > max {
                max = *x;
            }
        }
        return max;
    }

    pub fn update(&mut self, data: Word) {
        let (rx,ry) = self.robot_pos;
        let (x,y) = match &self.state {
            State::WaitResponse(dir ) => {
                match dir {
                    Direction::North => (rx, ry - 1),
                    Direction::South => (rx, ry + 1),
                    Direction::West => (rx - 1, ry),
                    Direction::East => (rx + 1, ry),
                }
            },
            _ => panic!(),
        };
        match data {
            0 => {
                self.update_tile(&x, &y, Wall);
                return;
            },
            1 => {
                self.update_tile(&x, &y, Empty);
                self.robot_pos = (x, y);
                let d = self.update_dist(x,y);
            },
            2 => {
                self.update_tile(&x, &y, OxygenSystem);
                self.robot_pos = (x, y);
                self.set_dist(x,y, 0);
                //println!("Dist: {}", d);
                //panic!("Done");

            },
            _ => panic!(),
        }
        self.update_dist(x,y);
    }

    pub fn print(&self) {
//        print!("{}[2J", 27 as char);
//        print!("\x1B[2J");


        for y in 0..self.y_size {
            for x in 0..self.x_size {
                let element;
                if (x, y) == self.robot_pos {
                    element = 'D';
                } else {
                    element = self.map[(y * self.x_size + x) as usize].borrow().repr();
                }
                print!("{}", element);
            }
            println!("");
        }
    }

}

fn main() {
    /*
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    */

    let filename = "data/day15/input.txt";
    let mut program = intmachine::read_program(filename);

    let mut controller:Controller = Controller::new(180, 180 );

    let (input, pin): (SyncSender<Message>, Receiver<Message>) = mpsc::sync_channel(0);
    let (pout, output): (SyncSender<Message>, Receiver<Message>) = mpsc::sync_channel(0);

    let child = thread::spawn(move || {
        intmachine::execute(&program, pin, pout);
    });

    let mut rng = rand::thread_rng();

    let mut count = 0;
    loop {
        if count % 10000 == 0 {
            println!("Max is: {}", controller.get_max_dist());
            //print!("{}[2J", 27 as char);
            //controller.print();
        }
        count += 1;

        match output.recv() {
            Ok(message) => {
                match message {
                    Message::Data(data) =>  {
                        controller.update(data);
                        controller.state = Input;
                    }

                    Message::Shutdown => break,
                    Message::RequestInput => {
                        let r = rng.gen_range(1, 5);
//                        println!("Random walk: {}", &r);
                        input.send(Message::Data(r));
                        controller.state = WaitResponse(Direction::from(r));
                    }
                }
          }
            Err(error) => {
                panic!("Error: {:?}", error.to_string());
            }
        }
    }
    child.join();

    controller.print();
//    println!("Score: {}", result);

}

#[cfg(test)]
mod tests {

}