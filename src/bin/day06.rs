use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

use std::collections::HashSet;
use std::collections::HashMap;
use regex::Regex;

extern crate regex;

fn find_node(orbits :&HashMap<String, HashSet<String>>, current_node: &str, node: &str) -> Option<Vec<String>> {
    if current_node == node {
        print!("Found node!");
        return Some(vec![String::from(current_node)])
    } else {
        println!("Current node: {}", current_node);
        let maybe_child_set = orbits.get(current_node);
        match maybe_child_set {
            Some(child_set) => {
                for child in child_set {
                    match find_node(&orbits, child, node) {
                        Some(path) => {
                            let mut ext = path.clone();
                            ext.push(String::from(current_node));
                            return Some(ext);
                        },
                        _ => {}
                    }
                }
            },
            _ => {}
        }
        return None
    }

}

fn construct_orbits(data :Vec<(String, String)>) -> HashMap<String, HashSet<String>> {
    let mut orbits : HashMap<String, HashSet<String>> = HashMap::new();

    for (p, c) in data {
        println!("Hmm: ");
        let parent_name = p.clone();
        let child_name = c.clone();

        let something = orbits.entry(String::from(p))
            .or_insert(HashSet::new());
        something.insert(String::from(c));

    }
    return orbits;
}

fn santa_distance(data :Vec<(String, String)>) -> i32 {

    let mut santa_dist : HashMap<String, i32> = HashMap::new();

    let orbits = construct_orbits(data);

    let santa_path = find_node(&orbits, "COM", "SAN");

    let mut dist = 0;

    for node in santa_path.unwrap() {
        match find_node(&orbits, &node, "YOU") {
            Some(path) => {
                println!("Found path: {:?}", path);
                return (dist - 1) + (path.len() as i32 - 2)
            }
            _ => {}
        }
        dist += 1;
    }

   return -1;
}

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
    /*
    let res = construct_starmap(orbits);
    let (a, b) = res;
    println!("Hm: {:?} {}", res, a+b)
    */

    let santa_d = santa_distance(orbits);
    println!("Santa distance: {}", santa_d);
}

fn parse_line(s :&str) -> (String, String) {
    let re :Regex = Regex::new(r#"^(.+)\)(.+)$"#).unwrap();
    let m = re.captures(s).unwrap();
    return (String::from(&m[1]), String::from(&m[2]));
}

#[cfg(test)]
mod tests {
    use crate::{parse_line, find_node, construct_orbits, santa_distance};

    #[test]
    fn test_parse() {
        assert_eq!(parse_line("AB)CD"), (String::from("AB"), String::from("CD")));
    }

    #[test]
    fn test_find_node() {
        let input =
            vec![
                (String::from("COM"), String::from("B")),
                (String::from("B"), String::from("C")),
                (String::from("C"), String::from("D")),
                (String::from("D"), String::from("E")),
                (String::from("E"), String::from("F")),
                (String::from("B"), String::from("G")),
                (String::from("G"), String::from("H")),
                (String::from("D"), String::from("I")),
                (String::from("E"), String::from("J")),
                (String::from("J"), String::from("K")),
                (String::from("K"), String::from("L")),
                (String::from("K"), String::from("YOU")),
                (String::from("I"), String::from("SAN")),
            ];

        let orbits = construct_orbits(input);

        let res = find_node(&orbits, "COM", "SAN");
        println!("Reul {:?}", res);
    }

    #[test]
    fn test_sana() {
        let input =
            vec![
                (String::from("COM"), String::from("B")),
                (String::from("B"), String::from("C")),
                (String::from("C"), String::from("D")),
                (String::from("D"), String::from("E")),
                (String::from("E"), String::from("F")),
                (String::from("B"), String::from("G")),
                (String::from("G"), String::from("H")),
                (String::from("D"), String::from("I")),
                (String::from("E"), String::from("J")),
                (String::from("J"), String::from("K")),
                (String::from("K"), String::from("L")),
                (String::from("K"), String::from("YOU")),
                (String::from("I"), String::from("SAN")),
            ];


        let res = santa_distance(input);
        println!("Reul {:?}", res);
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
