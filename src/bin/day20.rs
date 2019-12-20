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
use crate::Location::{Empty, Warp};
use petgraph::visit::{NodeFiltered, Dfs, GraphRef, NodeFilteredNodes, EdgeRef};
use std::ops::Index;
use petgraph::graph::{NodeIndex, Edge};
use petgraph::algo::{connected_components, dijkstra};
use crate::Portal::{InnerPortal, OuterPortal};

type Pos = (i32, i32);
type RPos = (Pos, i32);
type RawMap = Vec<Vec<char>>;

const MAX_DEPTH :i32 = 100;

#[derive(Eq, Copy, PartialEq, Clone, Hash, Debug)]
struct MapNode {
    x :i32,
    y :i32,
    level :i32,
    location: Location,
}

impl MapNode {
    fn get_pos(&self) -> Pos {
        return (self.x, self.y)
    }
    fn get_rpos(&self) -> RPos {
        return ((self.x, self.y), self.level)
    }
}

#[derive(Eq, Copy, PartialEq, Clone, Hash, Debug)]
enum Location {
    Empty,
    Warp,
}

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
enum Portal {
    InnerPortal(char, char, Pos),
    OuterPortal(char, char, Pos),
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Portals {
    inner_to_outer: HashMap<Pos, Portal>,
    outer_to_inner: HashMap<Pos, Portal>,
}

impl Portals {
    fn from_map(map: &RawMap) -> Portals {
        let ps = find_portals(map);
        println!("portals: {:?}", &ps);
        let mut outer_portals = HashMap::new();
        let mut inner_portals = HashMap::new();
        for p in ps.iter() {
            match p {
                OuterPortal(c1, c2, pos) => {
                    outer_portals.insert((c1,c2), p);
                },
                InnerPortal(c1, c2, pos) => {
                    inner_portals.insert((c1, c2), p);
                }
            }
        }
        let mut inner_to_outer: HashMap<Pos, Portal> = HashMap::new();
        let mut outer_to_inner: HashMap<Pos, Portal> = HashMap::new();
        for p in ps.iter() {
            match p {
                OuterPortal(c1, c2, pos) => {
                    let dest = inner_portals.get(&(c1, c2)).unwrap().clone().clone();
                    outer_to_inner.insert(*pos, dest);
                },
                InnerPortal(c1, c2, pos) => {
                    let dest = outer_portals.get(&(c1, c2)).unwrap().clone().clone();
                    inner_to_outer.insert(*pos, dest);
                }
            }
        }
        return Portals {
            inner_to_outer,
            outer_to_inner
        }
    }

    fn find_dest(&self, pos: RPos) -> Option<RPos> {
        let (pos_2d, level) = pos;
        let outer_portal = self.inner_to_outer.get(&pos_2d);
        let inner_portal = self.outer_to_inner.get(&pos_2d);
        return match (inner_portal, outer_portal, level) {
            (None, Some(OuterPortal(c1, c2, o_pos)), level) if level < MAX_DEPTH => {
                // Inside to out
                println!("Found inner to outer: {}{} ({})", c1, c2, level);
                Some((*o_pos, level + 1))
            },
            (Some(InnerPortal(c1, c2, o_pos)), None, level) if level > 0 => {
                println!("Found outer to inner: {}{} ({})", c1, c2, level);
                // Outside to in
                Some((*o_pos, level - 1))
            }
            _ => None,
        }
    }
}



fn main() {
    let filename = "data/day20/input.txt";
    let raw_map = read_file(filename);


    let ps = Portals::from_map(&raw_map);
    println!("{:?}", ps);

    let mut graph = Graph::<MapNode, i32, Undirected>::new_undirected();
    let mut pos_to_node = HashMap::new();

    let s_pos = find_place('A', &raw_map);
    let g_pos = find_place('Z', &raw_map);

    let mut q :VecDeque<RPos>= VecDeque::new();
    let mut visited = HashSet::new();

    q.push_back((s_pos, 0));
    q.push_back((g_pos, 0));

    while !q.is_empty() {
        let pos_3d = q.pop_front().unwrap();
        let (pos, level) = pos_3d;

        if visited.contains(&pos_3d) {
//            println!("Already visited: {:?}", &c);
            continue;
        }

        visited.insert(pos_3d);
        let current_node :NodeIndex;
        {
            current_node = pos_to_node.entry(pos_3d).or_insert_with(|| {
                let (c_x, c_y) = pos;
                assert_eq!(raw_map[c_y as usize][c_x as usize], '.');
                let new_node = graph.add_node(MapNode {
                    x: c_x,
                    y: c_y,
                    level,
                    location: Location::Empty
                });
                new_node
            }).clone();
        }

//        println!("We are at: {:?}", pos_3d);
        let maybe_portal = ps.find_dest(pos_3d);
        match maybe_portal {
            None => {},
            Some(dest) => {
                let existing_node = pos_to_node.get(&dest);
                match existing_node {
                    None => {
//                        println!("Found new node, adding: {:?}", dest);
                        q.push_back(dest);
                    },
                    Some(node) => {
 //                       println!("Found node, adding: {:?}", node);
                        graph.update_edge(current_node, *node, 1);
                    },
                }
            },
        }


        for nbd in vec![(0, -1), (0, 1), (1, 0), (-1,0)] {
            let nb = (pos.0 + nbd.0, pos.1 + nbd.1);
            let (x, y) = nb;
            let what = match raw_map[y as usize][x as usize] {
                '.' => {
                    Empty
                },
                '#' => continue,
                _ => {
                    continue;
                },
            };
            let gn = pos_to_node.get(&(nb, level));
            match gn {
                Some(node) => {
                    graph.update_edge(current_node, *node, 1);
                }
                None => {
                    let (x, y) = nb;
                    let new_data = MapNode {
                        x,
                        y,
                        level,
                        location: what,
                    };
                    let new_node = graph.add_node(new_data);
                    graph.add_edge(current_node, new_node, 1);
                    pos_to_node.insert((nb, level), new_node);
                }

            }
            q.push_back((nb, level));
        }


    }
    let goal = pos_to_node[&(g_pos, 0)];
    let start = pos_to_node[&(s_pos, 0)];

    let res =  find_goal_cost(start, goal, &graph);
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

fn find_portals(map: &RawMap) -> Vec<Portal> {
    let mut portal_set = HashSet::new();
    for (y, line) in map.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            let c :char = *char;
            if c.is_alphabetic() {
                let p = get_portal((x as i32, y as i32), map);

                match p {
                    InnerPortal(c1, c2, _) if c1 != c2 => {
                        portal_set.insert(p);
                    },
                    OuterPortal(c1, c2, _) if c1 != c2 => {
                        portal_set.insert(p);
                    },
                    _ => {
                        println!("Ignoring: {:?}", &p)
                    }
                }
            }
        }
    }
    return portal_set.iter().cloned().collect();
}

fn get_portal(pos: Pos, map: &RawMap) -> Portal {
    let (x1,y1) = pos;
    let l1 = map[y1 as usize][x1 as usize];
    let nbd :Vec<Pos> = vec![(0, 1), (0,-1), (1,0), (-1,0)];

    for d in nbd {
        let (x2, y2) = (pos.0 + d.0, pos.1 + d.1);
        if is_in_map((x2, y2), map) {
            let l2 = map[y2 as usize][x2 as usize];
            if l2.is_alphabetic() {
                let c1;
                let c2;
                if x2 > x1 || y2 > y1 {
                    c1 = l1;
                    c2 = l2;
                } else {
                    c1 = l2;
                    c2 = l1;
                }

                let after = (x2 + d.0, y2 + d.1);
                if is_in_map(after, map) && map[after.1 as usize][after.0 as usize] == '.' {
                    if is_outer_portal(after, map) {
                        return OuterPortal(c1, c2, after);
                    } else {
                        return InnerPortal(c1, c2, after);
                    }
                }
                let before = (x1 - d.0, y1 - d.1);
                if is_in_map(before, map) && map[before.1 as usize][before.0 as usize] == '.' {
                    if is_outer_portal(before, map) {
                        return OuterPortal(c1, c2, before);
                    } else {
                        return InnerPortal(c1, c2, before);
                    }
                }
            }
        }

    }
    panic!("Could not find portal location: {:?}", pos)
}

fn is_outer_portal(pos: Pos, map: &RawMap) -> bool {
    let (x, y) = pos;
    if y <=4 || y>= (map.len() as i32 - 4) {
        return true;
    }
    if x <=4 || x >= (map[y as usize].len() as i32 - 4) {
        return true;
    }
    return false;
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