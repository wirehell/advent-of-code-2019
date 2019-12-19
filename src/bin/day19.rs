use std::{env, thread};
use advent_of_code_2019::intmachine;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::{HashMap, VecDeque};
use std::sync::mpsc::{SyncSender, Receiver};
use advent_of_code_2019::intmachine::{Message, Word, execute_with_result, Memory};
use std::sync::mpsc;
use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;
use crate::State::{Input, WaitResponse};
use crate::Direction::{North, South, West, East};
use crate::Tile::{Pull, NoPull};
use std::cmp::Ordering;
use itertools::Itertools;


#[derive(Clone, Eq, PartialEq, Debug)]
enum Tile {
    Pull,
    NoPull,
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
    fn from(v :&Word) -> Tile {
        return match v {
            0 => NoPull,
            1 => Pull,
            _ => panic!("Unexpecteced value: {}", v),
        }
    }


    fn repr(&self) -> char {
        return match self {
            NoPull => '.',
            Pull => '#',
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
//    x :i64,
//    y :i64,
    map: Vec<Vec<Tile>>,
//    robot: Option<Robot>,
//    state: State,
}

fn main() {
    /*
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    */

    let filename = "data/day19/input.txt";
    let mut program = intmachine::read_program(filename);

    let mut rng = rand::thread_rng();

    let mut d :VecDeque<Word> = VecDeque::new();

    let x_size = 10;
    let y_size = 10;

    let mut count = 0;
    for y in 0..y_size {
        for x in 0..x_size {
            let result =  match is_tractor(&program, x, y)  {
                true => Tile::Pull,
                false => Tile::NoPull,
            };
            if result == Pull {
                count += 1;
            }
            print!("{}", result.repr());
        }
        println!("");
    }
    println!("Count is {}", count);

    /*
    for r in (10..100).step_by(1) {
        let res = find_box(&program, r, 2);
        println!("Ordering ({}): {:?}", r, res);
    }
    */
    //find_box(&program, 1728, 100);
    for r in (1700..1713).step_by(1) {
        let res = find_box(&program, r, 99);
        println!("Ordering ({}): {:?}", r, res);
    }
}


fn is_tractor(program: &Memory, x: Word, y: Word) -> bool {
    let result = execute_with_result(program, vec![x,y]);
    return result[0] == 1;
}

fn find_box(program: &Memory, row: Word, box_size: Word) -> Ordering {
    let x_max = 999999;
    // Find leftmost
    let mut x_start = 0;
    loop {
        if x_start >= x_max {
            panic!("Did not find beam on row: {}", row);
        }
        if is_tractor(program, x_start, row) {
            break;
        }
        x_start += 1;
    }
    let mut x_last = x_start;
    loop {
        if !is_tractor(program, x_last + 1, row) {
            break;
        }
        x_last += 1;
    }
    // Short circuit
    if x_last - x_start < box_size  {
        return Ordering::Less;
    }

    if is_tractor(program, x_last - (box_size + 1), row + box_size + 1) {
       return Ordering::Greater;
    }

    if is_tractor(program, x_last - box_size, row + box_size) {
        let x = x_last - box_size;
        println!("Point: {} {}", x, row);
        println!("Result: {}", x * 10000 + row);
        return Ordering::Equal;
    }

    return Ordering::Less;
}




#[cfg(test)]
mod tests {

}