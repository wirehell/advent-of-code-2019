use std::{env, thread};
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::{HashMap, VecDeque, HashSet, BinaryHeap};
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
use crate::Location::{Empty, Door, Key, Start};
use petgraph::visit::{NodeFiltered, Dfs, GraphRef, NodeFilteredNodes, EdgeRef};
use std::ops::Index;
use petgraph::graph::{NodeIndex, Edge};
use petgraph::algo::{connected_components, dijkstra};
use std::cmp::Ordering;
use std::hash::Hash;
use petgraph::stable_graph::{StableUnGraph, StableGraph, EdgeReference};
use geo::{Point, LineString, Polygon};
use geo::convexhull::ConvexHull;
use geo::algorithm::euclidean_length::EuclideanLength;
use petgraph::dot::{Dot, Config};


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

#[derive(Eq, PartialEq, Clone, Debug)]
struct KeySet {
    s: u32,
}

impl KeySet {
    fn new() -> KeySet {
        return KeySet {
            s: 0
        }
    }

    fn len(&self) -> u32 {
       return self.s.count_ones();
    }

    fn contains(&self, c :char) -> bool {
        assert!(c as u8 >= 97 && c as u8 <= 122);
//        println!("Checking for: {}", &c);
        let v = c as u32;
        let key = 1 << (v-97);
        return (self.s & key) != 0;
    }

    fn add(&mut self, c :char) {
//        println!("Adding: {}", &c);
        assert!(c as u8 >= 97 && c as u8 <= 122);
        let v = c as u32;
        let key = 1 << (v-97);
        self.s |= key;
    }
}

#[derive(Eq, Copy, PartialEq, Clone, Hash, Debug)]
enum Location {
    Start,
    Empty,
    Key(char),
    Door(char),
}

type RawMap = Vec<Vec<char>>;

fn main() {
//    let filename = "data/day18/test5.map";
    let filename = "data/day18/input.txt";
    let raw_map = read_file(filename);

    let mut keys = HashMap::new();

    let mut graph :StableGraph<MapNode, i32, Undirected> = StableGraph::default();
    let mut pos_to_node = HashMap::new();

    let (sx, sy) = find_myself(&raw_map);

    let mut q = VecDeque::new();
    let mut visited = HashSet::new();

    let initial = MapNode {
        x: sx,
        y: sy,
        location: Location::Start
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

    let mut changed = true;
    while changed == true {
        let c = graph.node_count();
//        println!("Reined: {}", c);
        changed = false;
        for ni in graph.node_indices() {
            let node = graph[ni];
            let edges = graph.edges(ni);
            let edg: Vec<EdgeReference<i32>> = edges.collect();

            match node.location {
                Empty => {
                    if edg.len() == 1 {
//                        println!("Removing node: {:?}", node);
                        graph.remove_node(ni);
                        changed = true;
                        break;
                    }

                    if edg.len() == 2 {
                        let nb1 = edg[0].target();
                        let nb2 = edg[1].target();
 //                       println!("Replacing node: {:?}", node);
                        graph.add_edge(nb1, nb2, edg[0].weight() + edg[1].weight());
                        graph.remove_node(ni);
                        changed = true;
                        break;
                    }

                },
                Start => {},
                Key(_) => {}, // Ignore for now
                Door(d) => {
                    if edg.len() == 1 {
                        println!("Removing useless door: {:?}", node);
                        graph.remove_node(ni);
                        changed = true;
                        break;
                    }

                },
            }
        }
    }


    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));


    let mut count = 0;
    let mut heap = BinaryHeap::new();
    let initial = Path {
        pos: start_node,
        p: vec![],
        ks: KeySet::new(),
        estimate: 0,
        cost: 0
    };

    heap.push(initial);
    let mut max_depth = 0;

    while !heap.is_empty() {
        let path = heap.pop().unwrap();
        if path.ks.len() as usize == keys.len() {
            println!("Found solution: {:?}", path);
            break;
        }
        max_depth = i32::max(max_depth, path.p.len() as i32);
        count += 1;
//        println!("Working on: {:?}", path);
        let reachable = find_reachable_keys(path.pos, &graph, &path.ks, &keys);
 //       println!("Reachable: {:?}", reachable);
        let rl = reachable.len();
        for (k, cost) in reachable {

            let pos = keys[&k];
            let previous = path.p.last();
            if previous.is_none() || allowed_path(*previous.unwrap(), k, &path.ks) {
                let mut new_ks: KeySet = path.ks.clone();
                new_ks.add(k);
//            let estimate = get_estimate_left(&graph, &new_ks, &keys);
                let estimate = 0;
//            println!("Estimate is: {}", estimate);
                let mut p = path.p.clone();
                p.push(k);
                let new_path: Path = Path {
                    pos,
                    ks: new_ks,
                    p,
                    estimate,
                    cost: path.cost + cost,
                };

                heap.push(new_path);

            }
        }

        if count % 10000 == 0 {
            println!("Current depth: {}, cost: {}, heap: {}, reachable: {}, max_depth: {}", 0, path.cost, heap.len(), rl, max_depth);
            println!("Path: {:?}", path);

        }
    }
    println!("Done.");
}

fn force_order(from :char, to:char, s: &Vec<char>) -> bool {
    let l = s.len();
    for i in 0..(l-1) {
        if to == s[i+1] && from != s[i] {
            return false;
        }
        if from == s[i] && to != s[i+1] {
            return false;
        }
    }
    return true;
}



fn allowed_path(from: char, to: char, ks: &KeySet) -> bool {
    let forced :Vec<Vec<char>> = vec![
        vec!['b','t','j'],
        vec!['r','w','s'],
        vec!['z', 'm','v'],
        vec!['f','o'],
        vec!['p', 'g', 'x', 'k', 'd', 'h']
    ];

    for f in forced {
        if force_order(from, to, &f) == false {
            return false;
        }
    }

    return true;
}



#[derive(Eq, PartialEq, Debug, Clone)]
struct Path {
    pos: NodeIndex,
    p: Vec<char>,
    ks: KeySet,
    estimate: i32,
    cost: i32,
}


impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        return (other.cost + other.estimate).cmp(&(self.cost + self.estimate));
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

fn get_estimate_left(graph: &StableGraph<MapNode, i32, Undirected, u32>, key_set: &KeySet, keys: &HashMap<char, NodeIndex>) -> i32 {

    let mut points = Vec::new();
    for (c, ni) in keys.iter() {
        if !key_set.contains(*c) {
            let mn :MapNode = graph[*ni];
            points.push(Point::new(mn.x as f64, mn.y as f64));
        }
    }
    if points.len() == 0 {
        return 0;
    }
    let linestring = LineString::from(points);
    let polygon = Polygon::new(linestring, vec![]);
    let hull = polygon.convex_hull();

    let line = hull.exterior().clone();
    return line.euclidean_length() as i32;
}


fn find_reachable_keys(start: NodeIndex, graph: &StableGraph<MapNode, i32, Undirected, u32>, key_set: &KeySet, keys: &HashMap<char, NodeIndex>) -> Vec<(char,i32)> {
    let mut reachable = vec![];

    let filtered = NodeFiltered(&graph, |node :NodeIndex| {
        return match graph[node].location {
            Start => {
                true
            }
            Empty => {
                true
            },
            Key(_) => {
                true
            },
            Door(d) => {
                key_set.contains(d)
            } ,
        }
    });
    let dist = dijkstra(&filtered, start, None, |e| *e.weight());
    for cv in 97..=122u8 {
        let c = cv as char;

        if !key_set.contains(c) {
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

    #[test]
    fn test_line_dis() {
        use geo::{Point, LineString, Coordinate};
        use geo::algorithm::euclidean_length::EuclideanLength;

        let mut vec = Vec::new();
        vec.push(Point::new(40.02f64, 116.34));
        vec.push(Point::new(42.02f64, 116.34));
        let linestring = LineString::from(vec);

        println!("EuclideanLength {}", linestring.euclidean_length());
    }

}