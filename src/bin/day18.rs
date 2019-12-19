use std::{env, thread};
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::{HashMap, VecDeque, HashSet};
use std::sync::mpsc::{SyncSender, Receiver};
use advent_of_code_2019::intmachine::{Message, Word};
use std::sync::mpsc;
use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;
use std::fs::File;
use std::io::{BufReader, BufRead};
use petgraph::{Graph, Undirected};
use crate::Location::{Empty, Door, Key};
use petgraph::visit::{NodeFiltered, Dfs, GraphRef, NodeFilteredNodes, EdgeRef};
use std::ops::Index;
use petgraph::graph::{NodeIndex, Edge};
use petgraph::algo::{connected_components, dijkstra};


#[derive(Eq, Copy, PartialEq, Clone, Hash, Debug)]
struct MapNode {
    x :i32,
    y :i32,
    location: Location,
}

impl MapNode {
    fn get_pos(&self) -> (i32, i32) {
        return (self.x, self.y)
    }
}

struct KeySet {
    s: HashSet<char>,
}

impl KeySet {
    fn new() -> KeySet {
        return KeySet {
            s: HashSet::new(),
        }
    }

    fn contains(&self, c :&char) -> bool {
        return self.s.contains(c);
    }

    fn add(&mut self, c :char) -> bool {
        return self.s.insert(c);
    }
}

#[derive(Eq, Copy, PartialEq, Clone, Hash, Debug)]
enum Location {
    Empty,
    Key(char),
    Door(char),
}

type RawMap = Vec<Vec<char>>;

fn main() {
    let filename = "data/day18/input.txt";
    let raw_map = read_file(filename);

    let mut keys = HashMap::new();

    let mut graph = Graph::<MapNode, i32, Undirected>::new_undirected();
    let mut pos_to_node = HashMap::new();

    let (sx, sy) = find_myself(&raw_map);

    let mut q = VecDeque::new();
    let mut visited = HashSet::new();

    let initial = MapNode {
        x: sx,
        y: sy,
        location: Location::Empty
    };
    let start_node = graph.add_node(initial.clone());
    pos_to_node.insert(initial.get_pos(), start_node.clone());

    q.push_back(initial.get_pos());

    while !q.is_empty() {
        let pos = q.pop_front().unwrap();
        if visited.contains(&pos) {
//            println!("Already visited: {:?}", &c);
            continue;
        }
        visited.insert(pos);
        let current_node;
        {
            current_node = pos_to_node.get(&pos).unwrap().clone();
        }
        let (x,y) = pos;

        for nb in vec![(x, y-1), (x, y+1), (x+1, y), (x-1,y)] {
            let (x, y) = nb;
            let b = raw_map[y as usize][x as usize];

            let what = match raw_map[y as usize][x as usize] {
                '#' => continue,
                '@' => continue,
                '.' => {
                    Empty
                },
                c if c.is_uppercase() => {
//                    println!("Found door: {}", c as char);
                    Door(c.to_ascii_lowercase())
                },
                c if c.is_lowercase() => {
//                    println!("Found key: {}", c as char);
                    Key(c)
                }
                _ => {
                    panic!("Unknown char at: {:?}", nb);
                }
            };
            let gn = pos_to_node.get(&nb);
            match gn {
                Some(node) => {
                    graph.update_edge(current_node, *node, 1);
                }
                None => {
                    let (x, y) = nb;
                    let new_data = MapNode {
                        x,
                        y,
                        location: what,
                    };
                    let new_node = graph.add_node(new_data);
                    match new_data.location {
                        Key(c) => {
                            keys.insert(c, new_node);
                        },
                        _ => {},
                    }
                    graph.add_edge(current_node, new_node, 1);
                    pos_to_node.insert(nb, new_node);

                }

            }
            q.push_back(nb);
        }
    }
    let mut ks = KeySet::new();
    let mut reachable = vec![];
    for i in 0..10000 {
        reachable = find_reachable_keys(start_node, &graph, &ks, &keys);
    }
    println!("{:?}", reachable);
}

fn find_reachable_keys(start: NodeIndex, graph: &Graph<MapNode, i32, Undirected, u32>, key_set: &KeySet, keys: &HashMap<char, NodeIndex>) -> Vec<(char,i32)> {
    let mut reachable = vec![];

    let filtered = NodeFiltered(&graph, |node :NodeIndex| {
        return match graph[node].location {
            Empty => {
                true
            },
            Key(_) => {
                true
            },
            Door(d) => {
                key_set.contains(&d)
            } ,
        }
    });
    let dist = dijkstra(&filtered, start, None, |e| *e.weight());
    for cv in 97..=122u8 {
        let c = cv as char;

        if !key_set.contains(&c) {
            let index = keys.get(&c);
            match index {
                None => {},
                Some(index) => {
                    match dist.get(index) {
                        None => {},
                        Some(d) => {
                            reachable.push((c, *d));
                        },
                    };
                }
            };
        }
    }

//    println!("hashmap: {:?}", dist);
    return reachable;

}

fn read_file(filename: &str) -> RawMap {
    let f = File::open(filename).expect("Could not open file");
    let file = BufReader::new(&f);
    let mut map = vec![];
    for line in file.lines() {
        let unwrapped  = line.unwrap();
        let pl :Vec<char> = unwrapped.trim().chars().collect();
        map.push(pl);
    }
    return map;
}

fn find_myself(map :&RawMap) -> (i32, i32) {
    for (y, line) in map.iter().enumerate() {
       for (x, char) in line.iter().enumerate() {
           if *char == '@' {
               return (x as i32, y as i32);
           }

       }
    }
    panic!("Could not find myself");
}



#[cfg(test)]
mod tests {
    use petgraph::Graph;
    use petgraph::graph::NodeIndex;
    use petgraph::visit::{NodeFiltered, DfsPostOrder, Dfs};
    use std::ops::Index;

    #[derive(Debug)]
    struct Tn {
        s: String,
    }
    impl Tn {
       fn from_str(s :&str) -> Tn {
           return Tn { s: s.to_string() }
       }
    }

    #[test]
    fn filtered() {
        let mut g = Graph::new_undirected();
        let a = g.add_node(Tn::from_str("A"));
        let b = g.add_node(Tn::from_str("B"));
        let c = g.add_node(Tn::from_str("C"));
        let d = g.add_node(Tn::from_str("D"));
        let e = g.add_node(Tn::from_str("E"));
        let f = g.add_node(Tn::from_str("F"));
        g.add_edge(a, b, 7);
        g.add_edge(c, a, 9);
        g.add_edge(a, d, 14);
        g.add_edge(b, c, 10);
        g.add_edge(d, c, 2);
        g.add_edge(d, e, 9);
        g.add_edge(b, f, 15);
        g.add_edge(c, f, 11);
        g.add_edge(e, f, 6);
        println!("{:?}", g);

        let filt = NodeFiltered(&g, |n: NodeIndex| n != c && !g.index(n).s.eq(&String::from("A")));

        let mut dfs = Dfs::new(&filt, f);
        let mut po = Vec::new();
        while let Some(nx) = dfs.next(&filt) {
            println!("Next: {:?}", nx);
            po.push(nx);
        }
    }

}