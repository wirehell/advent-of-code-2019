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

extern crate regex;


#[derive(Hash, Clone, Debug, Eq, PartialEq)]
struct System {
    pos: Vec<Position>,
    vel: Vec<Velocity>,
}

type Triplet = (i64, i64, i64);

fn add_tuple(a :Triplet, b :Triplet) -> Triplet {
    return (a.0 + b.0, a.1 + b.1, a.2 + b.2);
}

fn d(v1: i64, v2: i64) -> i64 {
    if v1 == v2 {
        return 0;
    } else if v1 > v2 {
        return -1;
    } else {
        return 1;
    }
}

fn calculate_gravity(p1: &Position, p2 :&Position) -> (Delta, Delta) {
    let dx = d(p1.0, p2.0);
    let dy = d(p1.1, p2.1);
    let dz = d(p1.2, p2.2);

    return ((dx,dy,dz), (-dx,-dy,-dz))
}


impl System {
    pub fn step(&mut self) {
        let it = (0..self.pos.len()).combinations(2);

        for c in it {
            let i1 = c[0];
            let i2 = c[1];
            let p1 = self.pos[i1];
            let p2 = self.pos[i2];
            let (d1, d2) = calculate_gravity(&p1, &p2);
            self.vel[i1] = add_tuple(self.vel[i1], d1);
            self.vel[i2] = add_tuple(self.vel[i2], d2);
        }

        for (i1, vel) in self.vel.iter().enumerate() {
            self.pos[i1] = add_tuple(self.pos[i1], *vel);
        }
    }

    fn slice_x(&mut self) -> (Vec<i64>, Vec<i64>) {
        let p = self.pos.iter().map(|v| v.0).collect();
        let v = self.vel.iter().map(|v| v.0).collect();
        return (p,v);
    }
    fn slice_y(&mut self) -> (Vec<i64>, Vec<i64>) {
        let p = self.pos.iter().map(|v| v.1).collect();
        let v = self.vel.iter().map(|v| v.1).collect();
        return (p,v);
    }
    fn slice_z(&mut self) -> (Vec<i64>, Vec<i64>) {
        let p = self.pos.iter().map(|v| v.2).collect();
        let v = self.vel.iter().map(|v| v.2).collect();
        return (p,v);
    }

    fn calculate_energy(&self) -> i64 {
        let mut sum = 0;
        for i in  0..self.pos.len() {
            sum += pot_energy(&self.pos[i]) * kin_energy(&self.vel[i]);
        }
        return sum;
    }

    fn ful_hash(&self) -> i32 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        let full = s.finish();
        let a = full & 0x0fffffff;
        return a as i32;
    }



}

fn calculate_steps(sys :System) -> i64 {
    let mut system = sys.clone();
//    let mut interesting_steps = vec![];
    let mut xc = HashSet::new();
    let mut yc = HashSet::new();
    let mut zc = HashSet::new();
    let mut count = 0;
    let mut x_cycle :Option<i64> = None;
    let mut y_cycle :Option<i64> = None;
    let mut z_cycle :Option<i64> = None;

    for i in 0..4000000 {
        /*
        println!("{} {:?} {:?} {:?}", i, system.slice_x(), y_cycle, z_cycle);
        stdout().flush();
        */
        if x_cycle.is_none() && !xc.insert(system.slice_x()) {
            x_cycle = Some(i.clone());
        }
        if y_cycle.is_none() && !yc.insert(system.slice_y()) {
            y_cycle = Some(i.clone());
        }
        if z_cycle.is_none() && !zc.insert(system.slice_z()) {
            z_cycle = Some(i.clone());
        }


        match (x_cycle, y_cycle, z_cycle) {
            (Some(x), Some(y), Some(z)) => {
                return lcm(lcm(x,y),z);
                break;
            }
            _ => {

            }
        }

        system.step();
    }

    return -1;
}

fn pot_energy(pos :&Position) -> i64 {
    return pos.0.abs() + pos.1.abs() + pos.2.abs();
}
fn kin_energy(vel :&Velocity) -> i64 {
    return vel.0.abs() + vel.1.abs() + vel.2.abs();
}

type Position = Triplet;
type Velocity = Triplet;
type Delta = Triplet;

fn read_file(filename :&str) -> System {

    let re :Regex = Regex::new(r"^<x=(-?\d+),\s*y=(-?\d+),\s*z=(-?\d+)\s*>$").unwrap();
    let f = File::open(filename).expect("Could not open file");
    let file = BufReader::new(&f);

    let mut system = System { pos: vec![], vel: vec![]};
    for line in file.lines() {

        let l = line.unwrap();
        println!("line: {}", l);

        let m = re.captures(l.trim()).unwrap();
        let p = (m[1].parse().unwrap(), m[2].parse().unwrap(), m[3].parse().unwrap());
        system.pos.push(p);
        system.vel.push((0, 0, 0));
    }
    return system;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut system = read_file(filename);

    let sys = system.clone();

    for step in 0..1000 {
        system.step();
        println!("{:?}", &system)
    }

    let energy = system.calculate_energy();
    println!("Energy: {}", energy);

    let steps = calculate_steps(sys);
    println!("Steps: {}", steps);


}
#[cfg(test)]
mod tests {
    use crate::{read_file, System, calculate_steps};

    #[test]
    fn test_read_system() {
        let system = read_file("./data/day12/example.txt");
        assert_eq!(system, System {
            pos: vec![
                (-1, 0, 2),
                (2,-10, -7),
                (4,-8,8),
                (3,5,-1),
            ],
            vel: vec![
                (0, 0 ,0),
                (0, 0 ,0),
                (0, 0 ,0),
                (0, 0 ,0),
            ]
        });
    }

    #[test]
    fn test_step() {
        let mut system = read_file("./data/day12/example.txt");
        system.step();
        assert_eq!(system, System {
            pos: vec![
                (2, -1, 1),
                (3,-7, -4),
                (1,-7,5),
                (2,2,0),
            ],
            vel: vec![
                (3, -1 ,-1),
                (1, 3 ,3),
                (-3, 1 ,-3),
                (-1, -3 ,1),
            ]
        });
    }

    #[test]
    fn test_energy() {
        let mut system = read_file("./data/day12/example.txt");
        for i in 0..10 {
            system.step();
        }
        let energy = system.calculate_energy();
        assert_eq!(energy, 179);

    }

    #[test]
    fn test_calc_steps() {
        let mut system = read_file("./data/day12/example.txt");
        let steps = calculate_steps(system);
        assert_eq!(steps, 2772);
    }

}