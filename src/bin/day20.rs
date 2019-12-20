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
use crate::Location::{Empty, Portal};
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


#[derive(Eq, Copy, PartialEq, Clone, Hash, Debug)]
enum Location {
    Empty,
    Portal(char, char),
}

type RawMap = Vec<Vec<char>>;

fn main() {
    let filename = "data/day20/input.txt";
    let raw_map = read_file(filename);

    let mut portals = HashMap::new();

    let mut graph = Graph::<MapNode, i32, Undirected>::new_undirected();
    let mut pos_to_node = HashMap::new();

    let (sx, sy) = find_place('A', &raw_map);
    let (gx, gy) = find_place('Z', &raw_map);

    let mut q = VecDeque::new();
    let mut visited = HashSet::new();

    let initial = MapNode {
        x: sx,
        y: sy,
        location: Location::Empty
    };
    let start_node = graph.add_node(initial.clone());
    pos_to_node.insert(initial.get_pos(), start_node.clone());


    for (y, line) in raw_map.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if *char == '.' {
                q.push_back((x as i32, y as i32));
            }

        }
    }

    while !q.is_empty() {
        let pos = q.pop_front().unwrap();

        if visited.contains(&pos) {
//            println!("Already visited: {:?}", &c);
            continue;
        }
        visited.insert(pos);
        let current_node :NodeIndex;
        {
            current_node = pos_to_node.entry(pos).or_insert_with(|| {
                let (nx, ny) = pos;
                assert_eq!(raw_map[ny as usize][nx as usize], '.');
                let new_node = graph.add_node(MapNode {
                    x: nx,
                    y: ny,
                    location: Location::Empty
                });
                new_node
            }).clone();
        }

        println!("We are at: {:?}", pos);
        for nbd in vec![(0, -1), (0, 1), (1, 0), (-1,0)] {
            let nb = (pos.0 + nbd.0, pos.1 + nbd.1);

            let (x, y) = nb;

            let what = match raw_map[y as usize][x as usize] {
                '#' => continue,
                '.' => {
                    Empty
                },
                c if c.is_uppercase() => {
                    println!("Found portal: {}", c as char);
                    let (x2, y2) = (nb.0 + nbd.0, nb.1 + nbd.1);
                    let second_char = raw_map[y2 as usize][x2 as usize];
                    let portal;
                    if x2 > x || y2 > y {
                        portal = Portal(c, second_char)
                    } else {
                        portal = Portal(second_char, c)
                    }
                    graph[current_node].location = portal; // update current location
                    let other = portals.get(&portal);
                    match other {
                        None => {
                            println!("Inserting portal: {:?}", &portal);
                            portals.insert(portal, current_node);
                        },
                        Some(node) => {
                            println!("Inserting link between portals: {:?}-{:?}", &portal, &other);
                            graph.add_edge(current_node, *node, 1);
                        },
                    }
                    continue;
                },
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
                    graph.add_edge(current_node, new_node, 1);
                    pos_to_node.insert(nb, new_node);

                }

            }
            q.push_back(nb);
        }
    }

    let goal = pos_to_node[&(gx,gy)];

    let res =  find_goal_cost(start_node, goal, &graph);
    println!("{:?}", res);
}

fn find_goal_cost(start: NodeIndex,  goal: NodeIndex, graph: &Graph<MapNode, i32, Undirected, u32>) -> i32 {

    let dist = dijkstra(&graph, start, Some(goal), |e| *e.weight());
    return *dist.get(&goal).unwrap();
}

fn read_file(filename: &str) -> RawMap {
    let f = File::open(filename).expect("Could not open file");
    let file = BufReader::new(&f);
    let mut map = vec![];
    for line in file.lines() {
        let unwrapped  = line.unwrap();
        let pl :Vec<char> = unwrapped.chars().collect();
        map.push(pl);
    }
    return map;
}

type Pos = (i32, i32);

fn is_in_map(pos :Pos, map: &RawMap) -> bool {
    let (x,y) = pos;
    if y < 0 || x < 0 {
        return false;
    }
    if y >= map.len() as i32 {
        return false;
    }
    if x >= map[y as usize].len() as i32{
        return false;
    }
    return true;

}

// Return the position of the portal from one of the portal letter locations
fn exact_portal_location(pos: Pos, map: &RawMap) -> Pos {
    let (x1,y1) = pos;
    let letter = map[y1 as usize][x1 as usize];
    let nbd :Vec<Pos> = vec![(0, 1), (0,-1), (1,0), (-1,0)];

    for d in nbd {
        let (x2, y2) = (pos.0 + d.0, pos.1 + d.1);
        if is_in_map((x2, y2), map) {
            if map[y2 as usize][x2 as usize] == letter {
                let after = (x2 + d.0, y2 + d.1);
                if is_in_map(after, map) && map[after.1 as usize][after.0 as usize] == '.' {
                    return after;
                }
                let before = (x1 - d.0, y1 - d.1);
                if is_in_map(before, map) && map[before.1 as usize][before.0 as usize] == '.' {
                    return before;
                }
            }
        }

    }
    panic!("Could not find portal location: {:?}", pos)

}

fn find_place(portal_letter: char, map :&RawMap) -> (i32, i32) {
    for (y, line) in map.iter().enumerate() {
       for (x, char) in line.iter().enumerate() {
           for nbd in vec![(1,0), (-1,0), (0,1), (0,-1)] {
               let nb :(i32, i32) = (x as i32 + nbd.0, y as i32 + nbd.1);
               if *char == portal_letter && is_in_map(nb, map) && map[nb.1 as usize][nb.0 as usize] == portal_letter {
                   return exact_portal_location((x as i32, y as i32), map);
               }
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