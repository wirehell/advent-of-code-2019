use std::env;
use std::io::{BufReader, BufRead, stdout, Write};
use std::fs::File;
use regex::Regex;

use itertools::Itertools;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::ops::BitAnd;
use std::collections::{HashMap, HashSet};
use num::integer::lcm;
use std::borrow::Borrow;

extern crate regex;

type Level = i32;
const X_SIZE :u32 = 5;
const Y_SIZE :u32 = 5;

struct Gol {
    cells: Vec<u32>,
    l_max: i32,
    l_min: i32,
}

fn bit_rep(x :u32, y :u32) -> u32 {
    return 1 << (y * X_SIZE + x);
}

impl Gol {
    fn new(cells: Vec<Vec<bool>>) -> Gol {
        let mut val = 0;
        for (y, l) in cells.iter().enumerate() {
            for (x, c)in l.iter().enumerate() {
                if *c {
                    val |= bit_rep(x as u32, y as u32)
                }
            }
        }

        return Gol {
            cells: vec![val],
            l_max: 0,
            l_min: 0,
        }
    }

    fn bug_count(&self) -> u32 {
        return self.cells.iter().cloned().map(u32::count_ones).sum();
    }


    fn evolve(&mut self) {
        let mut new = vec![0; self.cells.len() + 2];
        for level in self.l_min-1..=self.l_max+1 {
            for y in 0..Y_SIZE {
                for x in 0..X_SIZE {
                    if (x==2) && (y==2) {
                        continue;
                    }
                    let br = bit_rep(x, y);

                    let c= self.get_cell(x, y, level);
                    let li = (level - self.l_min + 1) as usize;

                    let nc = self.get_neighbour_count(x as u32, y as u32, level);
//                    println!("x: {} y: {} br: {} li: {} nc: {}", x, y, br, li, nc);
                    if (c != 0) && !(nc == 1) {
                        new[li] &= !br; // Clear
                    } else if (c == 0) && (nc == 1 || nc == 2) {
                        new[li] |= br; // Set
                    } else {
                        new[li] |= c; // Copy previous state
                    }
                }
            }
        }
        self.cells = new;
        self.l_min -= 1;
        self.l_max += 1;
    }

    fn get_neighbour_count(&self, x: u32, y: u32, level: Level) -> u32 {
        let mut count = 0;
        let mut neighbours = vec![];
        if x == 0 {
            neighbours.push((1, 2, level-1));
        }
        if x == 4 {
            neighbours.push((3, 2, level-1));
        }
        if y == 0 {
            neighbours.push((2, 1, level-1));
        }
        if y == 4 {
            neighbours.push((2, 3, level-1));
        }

        if x == 1 && y == 2 {
            for i in 0..Y_SIZE {
                neighbours.push((0,i, level+1))
            }
        }
        if x == 3 && y == 2 {
            for i in 0..Y_SIZE {
                neighbours.push((4,i, level+1))
            }
        }
        if x == 2 && y == 1 {
            for i in 0..X_SIZE {
                neighbours.push((i,0, level+1))
            }
        }
        if x == 2 && y == 3 {
            for i in 0..X_SIZE {
                neighbours.push((i,4, level+1))
            }
        }
        if x < 4 {
            if !(x == 1 && y == 2) {
                neighbours.push ((x + 1, y, level))
            }
        }
        if x > 0 {
            if !(x == 3 && y == 2) {
                neighbours.push ((x - 1, y, level))
            }
        }
        if y < 4 {
            if !(x == 2 && y == 1) {
                neighbours.push ((x, y + 1, level))
            }
        }
        if y > 0 {
            if !(x == 2 && y == 3) {
                neighbours.push ((x, y - 1, level))
            }
        }
        return neighbours.iter()
            .map(|(x, y, l)| self.get_cell(*x, *y, *l))
            .map(|x| {
                match x {
                    0 => 0,
                    _ => 1,
                }
            })
            .sum()
    }

    fn print(&self) {
        for level in self.l_min..=self.l_max {
            println!("Level: {}", level);
            for y in 0..Y_SIZE {
                for x in 0..X_SIZE {
                    if x==2 && y==2 {
                        print!("?");
                    } else {
                        let c = self.get_cell(x, y, level);
                        if c != 0 {
                            print!("#");
                        } else {
                            print!(".");
                        }
                    }
                }
                println!("");
            }
        }
        println!("--")
    }

    fn get_cell(&self, x: u32, y:u32, l: Level) -> u32 {
        let li = (l - self.l_min) as usize;
        if li < 0 || li >= self.cells.len() {
            return 0;
        } else {
            return self.cells[li] & bit_rep(x, y);
        }
    }

}


fn read_file(filename :&str) -> Gol {

    let f = File::open(filename).expect("Could not open file");
    let file = BufReader::new(&f);

    let mut cells = vec![];
    for line in file.lines() {

        let l = line.unwrap();
        let trimmed = l.trim();

        let x :Vec<bool> = trimmed.chars().map(|x| {
            match x {
                '#' => true,
                '.' => false,
                _ => panic!(),
            }
        }).collect();
        cells.push(x);
    }

    return Gol::new(cells);
}


fn main() {
//    let args: Vec<String> = env::args().collect();
//    let filename = &args[1];

//    let mut gol = read_file(filename);
    let mut gol = read_file("data/day24/input.txt");
    println!("Initial: ");
    gol.print();
    let mut seen = HashSet::<u32>::new();

    for step in 0..200 {
        gol.evolve();
        gol.print();
    }

    println!("Sum is: {}", gol.bug_count());

}

#[cfg(test)]
mod tests {

}