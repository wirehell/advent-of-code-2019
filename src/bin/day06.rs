use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

use std::collections::HashSet;
use std::collections::HashMap;
use regex::Regex;
use std::borrow::{Borrow, BorrowMut};

extern crate regex;

fn construct_starmap(data :Vec<(String, String)>) -> (i64, i64){
    let mut orbits : HashMap<String, HashSet<String>> = HashMap::new();

    for (p, c) in data {
        println!("Hmm: ");
        let parent_name = p.clone();
        let child_name = c.clone();

        let something = orbits.entry(String::from(p))
            .or_insert(HashSet::new());
        something.insert(String::from(c));

    }

    let mut direct = 0;
    let mut indirect = 0;

    let mut current = &String::from("COM");
    let mut queue :Vec<(&String, i64)> = vec![];
    let mut depth = 0;

    println!("Hmm: {:?}", orbits);
    loop {
        println!("Planet: {} at depth {}", current, depth);
        match orbits.get(current) {
            Some(set) => {
                for item in set {
                    queue.push((item, depth + 1));
                    direct += 1;
                }
                indirect += i64::max(depth-1, 0) // For COM

            }
            None => {
                indirect += depth-1;
            }
        }

        if !queue.is_empty() {
            let (new_current, new_depth) = queue.pop().unwrap();
            current = new_current;
            depth = new_depth;

        } else {
            return (direct, indirect);
        }

    }

}


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Reading from file: {}", filename);
    let f = File::open(filename).expect("Oops! Something went wrong");
    let file = BufReader::new(&f);

    let mut orbits: Vec<(String, String)> = vec![];
    for line in file.lines() {
        orbits.push(parse_line(line.unwrap().trim()));
    }
    let res = construct_starmap(orbits);
    let (a, b) = res;

    println!("Hm: {:?} {}", res, a+b)
}

fn parse_line(s :&str) -> (String, String) {
    let re :Regex = Regex::new(r#"^(.+)\)(.+)$"#).unwrap();
    let m = re.captures(s).unwrap();
    return (String::from(&m[1]), String::from(&m[2]));
}

#[cfg(test)]
mod tests {
    use crate::{construct_starmap, parse_line};

    #[test]
    fn test_parse() {
        assert_eq!(parse_line("AB)CD"), (String::from("AB"), String::from("CD")));
    }

    #[test]
    fn test() {
        println!("Test");
        /*
        println!("Reul {:?}", construct_starmap(vec![
            ("COM", "B"),
            ("B", "C"),
            ("C", "D"),
            ("D", "E"),
            ("E", "F"),
            ("B", "G"),
            ("G", "H"),
            ("D", "I"),
            ("E", "J"),
            ("J", "K"),
            ("K", "L"),
        ]));
        */
    }
}
/*
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
*/
