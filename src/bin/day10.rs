use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;
use std::collections::{HashSet, HashMap, BinaryHeap};
use std::cmp::{Ordering, Reverse};
use std::f64::consts::PI;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
use std::ops::Deref;

#[derive(Debug, Eq, PartialEq)]
enum Space {
    Empty,
    Asteroid,
}
type Map = Vec<Vec<Space>>;

type Pos = (i32, i32);
type Dir = (i32, i32);

fn read_file(filename: &str) -> Map {
    println!("Reading from file: {}", filename);
    let f = File::open(filename).expect("Could not open file");
    let file = BufReader::new(&f);

    let mut map = vec![];

    for line in file.lines() {
        let mut lmap = vec![];
        for c in line.unwrap().trim().chars() {
            if c == '#' {
                lmap.push(Space::Asteroid);
            } else if c == '.' {
                lmap.push(Space::Empty);
            }
            else {
                panic!("Invalid map");
            }
        }
        map.push(lmap);

    }
    //println!("Map: {:?}", map);

    return map
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let map = read_file(filename);
    let (result, pos) = solve(&map);
    println!("Result: {} {:?}", result, pos);

    let (x, y) = pos;
    let ast = find_vaporization_point(&map, x, y, 200);
    println!("Asteroid: {:?}", ast);
}

fn gcd(mut m: i32, mut n: i32) -> i32 {
   while m != 0 {
       let old_m = m;
       m = n % m;
       n = old_m;
   }
   n.abs()
}

#[derive(Clone, PartialEq, Debug)]
pub struct Asteroid {
    pub pos : Pos,
    pub dist : f64,
}

impl Eq for Asteroid {

}

impl PartialOrd for Asteroid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return other.dist.partial_cmp(self.dist.borrow());
    }
}

impl Ord for Asteroid {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.partial_cmp(other).unwrap();
    }
}

struct AsteroidLine {
    heap : Rc<RefCell<BinaryHeap<Asteroid>>>,
}

impl AsteroidLine {
    fn new() -> AsteroidLine {
        return AsteroidLine {
            heap: Rc::new(RefCell::new(BinaryHeap::new()))
        };
    }
    fn push(&self, asteroid: Asteroid) {
        self.heap.borrow_mut().push(asteroid);
    }
    fn pop(&self) -> Option<Asteroid> {
        return self.heap.borrow_mut().pop()
    }


}

fn find_vaporization_point(map :&Map, x:i32, y:i32, n:i32) -> Asteroid {
    let vaporization = vaporize(map, x, y);
    let a  = vaporization[(n-1) as usize].borrow();
    return a.clone();
}

fn vaporize(map :&Map, x:i32, y:i32) -> Vec<Asteroid> {
    let mut ocupied_directions :HashMap<Dir, AsteroidLine> = HashMap::new();

    //println!("--");
    //println!("Examining: {} {}", x, y);

    for (y_ci, line_c) in map.iter().enumerate() {
        let y_c = y_ci as i32;
        for (x_ci, element_c) in line_c.iter().enumerate() {
            let x_c = x_ci as i32;
            if *element_c == Space::Asteroid && !(x_c == x && y_c == y) {

                let diff_x = x_c - x;
                let diff_y = y_c - y;
                //println!("Comparing: x:{} x_c:{} y:{} y_c:{}", x, x_c, y, y_c);

                let g = gcd(diff_x, diff_y);

                let dx = diff_x / g;
                let dy = diff_y / g;
                let dir = (dx, dy);
                //println!("Direction is {:?}", &dir);
                let xf = diff_x as f64;
                let yf = diff_y as f64;
                let dist = (xf * xf + yf * yf).sqrt();

                let asteroid_line = ocupied_directions.entry(dir).or_insert(AsteroidLine::new());
                asteroid_line.push(Asteroid { pos: (x_c as i32, y_c as i32), dist });

            }
        }
    }

    let mut directions :Vec<Pos> = vec![];
    for key in ocupied_directions.keys() {
        directions.push(*key);
    }
    directions.sort_by(|p1,p2| rotation_order(p1, p2));


    let mut vap_order = vec![];
    loop {
        let mut has_more = false;
        for direction in directions.iter() {
            let m_line = ocupied_directions.get(&direction);
            match m_line {
                None => {}
                Some(line) => {
                    let m_ast = line.pop();
                    match m_ast {
                        None => { }
                        Some(ast) => {
                            vap_order.push(ast);
                            has_more = true;
                        }
                    }
                }
            }
        }

        if !has_more {
            break;
        }
    }

    return vap_order;
}

fn rotation_order(p1 :&Pos, p2 :&Pos) -> Ordering {
    if p1 == p2 {
        return Ordering::Equal;
    }

    let a1 = angle(&p1);
    let a2 = angle(&p2);

    return a1.partial_cmp(&a2).unwrap();
}

fn solve(map :&Vec<Vec<Space>>) -> (i32, Pos) {
//    let mut directions;
 //   let mut count;
    let mut max = 0;
    let mut max_pos :Option<Pos> = None;

    for (y, line) in map.iter().enumerate() {
        for (x, element) in line.iter().enumerate() {
            let mut count = 0;
            let mut ocupied_directions = HashSet::new();

            //println!("--");
            //println!("Examining: {} {}", x, y);

            for (y_c, line_c) in map.iter().enumerate() {
                for (x_c, element_c) in line_c.iter().enumerate() {
                    if *element_c == Space::Asteroid && !(x_c == x && y_c == y) {
                        let diff_x = x_c as i32 - x as i32;
                        let diff_y = y_c as i32 - y as i32;
                        //println!("Comparing: x:{} x_c:{} y:{} y_c:{}", x, x_c, y, y_c);

                        let g = gcd(diff_x, diff_y);

                        let dx = diff_x / g;
                        let dy = diff_y / g;
                        //println!("Direction is {:?}", (dx, dy));

                        if ocupied_directions.insert((dx, dy)) {
                            //println!(" ->yes");
                            count = count + 1;
                        }
                    }
                }
            }
            if *element == Space::Asteroid {
                if count > max {
                    max_pos = Some((x as i32, y as i32));
                    max = count;
                }
            }
        }
    }

    return (max, max_pos.unwrap());
}

fn angle(p :&Pos) -> f64 {
    let (x, y) = p;
    let xf = *x as f64;
    let yf = *y as f64;
    let mut a =  yf.atan2(xf) + PI/2.0;
    if a < 0.0 {
        a += 2.0*PI;
    }
    return a;
}

#[cfg(test)]
mod tests {
    use crate::{read_file, gcd, solve, angle, rotation_order, vaporize, Asteroid, AsteroidLine, find_vaporization_point};
    use std::f64::consts::PI;
    use std::cmp::Ordering;


    /*
    #[test]
    fn test_vaporize() {
        let map = read_file("./data/day10/vap.txt");
        let res = vaporize(&map, 8, 3);
        println!("Res: {:?}", res);
    }
    */
    #[test]
    fn test_vaporize_big() {
        let map = read_file("./data/day10/testbig.txt");
        let x = find_vaporization_point(&map, 11, 13, 200);
        println!("Res: {:?}", x);
    }


    #[test]
    fn test_gcd() {
        assert_eq!(gcd(-4,2), 2);
        assert_eq!(gcd(-4,0), 4);
    }

    /*
    #[test]
    fn test_asteroid_line() {
        let a1 :Asteroid = Asteroid { pos: (0, 0), dist: 2.0 };
        let a2 :Asteroid = Asteroid { pos: (0, 0), dist: 3.0 };
        let a3 :Asteroid = Asteroid { pos: (0, 0), dist: 1.0 };
        let a4 :Asteroid = Asteroid { pos: (0, 0), dist: 4.0 };

        let mut line = AsteroidLine::new();
        line.add(a1);
        line.add(a2);
        line.add(a3);
        line.add(a4);

        loop {
            let b = line.pop();
            match b {
                Some(asteroid) => {
                    println!("Asteroid: {:?}", asteroid);

                }
                None => break,
            }
        }

    }
    */

    #[test]
    fn test_angle() {
        /*
        println!("{}",angle(&(0,-1)));
        println!("{}",angle(&(1,0)));
        println!("{}",angle(&(0,1)));
        println!("{}",angle(&(-1,0)));
        */
        assert_eq!(angle(&(0,-1)), 0.0); // UP
        assert_eq!(angle(&(1,0)), PI/2.0); // RIGHT
        assert_eq!(angle(&(0,1)), PI); // DOWN
        assert_eq!(angle(&(-1,0)), 3.0*PI/2.0); // LEFT
    }

    #[test]
    fn test_rotation_order() {
        assert_eq!(rotation_order(&(1,0), &(-1, 1)), Ordering::Less);
    }

    #[test]
    fn test_simple() {
        let map = read_file("./data/day10/test1.txt");
        assert_eq!(solve(&map), (8, (3,4)));
    }
    #[test]
    fn test_big() {
        let map = read_file("./data/day10/testbig.txt");
        assert_eq!(solve(&map), (210, (11, 13)));
    }

}
