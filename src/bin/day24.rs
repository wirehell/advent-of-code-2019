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

struct Gol {
    cells: Vec<Vec<bool>>,
    x_size: usize,
    y_size: usize,
}

impl Gol {
    fn new(cells: Vec<Vec<bool>>) -> Gol {
        let x_size = cells[0].len();
        let y_size = cells.len();
        return Gol {
            cells,
            x_size,
            y_size
        }
    }
    fn evolve(&mut self) {
        let mut new = self.cells.clone();
        for (y, yv) in self.cells.iter().enumerate() {
            for (x, c) in yv.iter().enumerate() {
                let nc = self.get_neighbour_count(x as u32, y as u32);
                if *c && !(nc == 1) {
                    new[y][x] = false;
                } else if !*c && (nc == 1 || nc == 2) {
                    new[y][x] = true;
                } else {
                    new[y][x] = self.cells[y][x];
                }
            }
        }
        self.cells = new;
    }

    fn hash(&self) -> u32 {
        let mut n = 0;
        let mut res = 0;
        for y in self.cells.iter() {
            for x in y {
                if *x {
                    res += 1 << n;
                }
                n+=1;
            }
        }
        return res;
    }

    fn get_neighbour_count(&self, x: u32, y: u32) -> u32 {
        let mut count = 0;
        let nb:Vec<(i32, i32)> = vec![
            (x as i32 -1,y as i32 ),
            (x as i32 ,y as i32 -1), (x as i32 , y as i32 +1),
            (x as i32 +1,y as i32 )
        ];

        return nb.iter()
            .map(|(x, y)| self.get_cell(*x, *y))
            .map(|x| {
                match x {
                    Some(true) => 1,
                    _ => 0,
                }
            })
            .sum()
    }

    fn print(&self) {
        for y in self.cells.iter() {
            for x in y {
                if *x {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
        println!("--")
    }

    fn get_cell(&self, x: i32, y:i32) -> Option<bool> {
        if x < 0 || y < 0 || x >= self.x_size as i32 || y >= self.y_size as i32 {
            return None;
        }
        return Some(self.cells[y as usize][x as usize]);
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
    seen.insert(gol.hash());

    for step in 0..100 {
        gol.evolve();
        let hash = gol.hash();
        gol.print();
        if !seen.insert(hash) {
            println!("Found: {:?}", hash);
            break;
        }
    }

}

#[cfg(test)]
mod tests {

}