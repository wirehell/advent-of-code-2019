use std::env;
use std::io::{BufReader, BufRead};
use std::fs::File;
use regex::Regex;
use crate::Dir::{Vertical, Horizontal};
use std::process::exit;

extern crate regex;

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}
const ORIGIN :Point = Point {x:0, y:0};

struct Line {
    from: Point,
    to: Point,
    dir: Dir,
    len: i32,
}
enum Dir {
    Vertical, Horizontal
}

type Wire = Vec<Line>;

enum Ext {
    U(i32),
    D(i32),
    L(i32),
    R(i32),
}

fn wire_length(wire: &Wire) -> i32 {
    return wire.last().map(|l| l.len).unwrap_or(0)
}

fn extend_wire(wire: &mut Wire, extension: Ext) {

    let last_point = wire.last().map_or(ORIGIN, |l| l.to);

    let new_point = match extension {
        Ext::U(n) => Point { x: last_point.x, y: last_point.y + n },
        Ext::D(n) => Point { x: last_point.x, y: last_point.y - n },
        Ext::R(n) => Point { x: last_point.x + n, y: last_point.y },
        Ext::L(n) => Point { x: last_point.x - n, y: last_point.y },
    };

    let (dir, len) = match extension {
        Ext::U(l)|Ext::D(l) => (Vertical,l),
        Ext::R(l) | Ext::L(l) => (Horizontal,l)
    };

    let len = wire_length(&wire) + len;

//    println!("NP: {} {}", new_point.x, new_point.y);
    wire.push(Line {from: last_point, to: new_point, dir, len:len });
}

fn parse_extension(s :&str) -> Ext {
    let re :Regex = Regex::new(r"^([UDRL])(\d+)$").unwrap();
    let m = re.captures(s).unwrap();
    return match m[1].as_ref() {
        "U" =>  Ext::U(m[2].parse().unwrap()),
        "D" => Ext::D(m[2].parse().unwrap()),
        "R" => Ext::R(m[2].parse().unwrap()),
        "L" => Ext::L(m[2].parse().unwrap()),
        x => panic!("Wtf {}", x)
    }
}

fn get_intersection_2(h_line: &Line, v_line:&Line) -> Option<Point> {
    let x = v_line.from.x;
    let y = h_line.from.y;

    let x_min = i32::min(h_line.from.x, h_line.to.x);
    let x_max = i32::max(h_line.from.x, h_line.to.x);

    let y_min = i32::min(v_line.from.y, v_line.to.y);
    let y_max = i32::max(v_line.from.y, v_line.to.y);

    if x > x_min && x < x_max && y > y_min && y < y_max {
        return Some(Point {x,y})
    }

    return None
}

fn get_intersection_distance(h_line: &Line, v_line:&Line) -> Option<i32> {
    let x = v_line.from.x;
    let y = h_line.from.y;

    let x_min = i32::min(h_line.from.x, h_line.to.x);
    let x_max = i32::max(h_line.from.x, h_line.to.x);

    let y_min = i32::min(v_line.from.y, v_line.to.y);
    let y_max = i32::max(v_line.from.y, v_line.to.y);

    if x > x_min && x < x_max && y > y_min && y < y_max {
        println!("Before {} {}", h_line.len, v_line.len);
        println!("x: {} y: {}", x, y);
        return Some(h_line.len + v_line.len
            - i32::abs(x - h_line.to.x) - i32::abs(y - v_line.to.y))
    }

    return None
}

fn get_intersection(line1 :&Line, line2:&Line) -> Option<i32> {
    return match (&line1.dir, &line2.dir) {
        (Horizontal, Vertical) => get_intersection_distance(line1, line2),
        (Vertical, Horizontal) =>  get_intersection_distance(line2, line1),
        _ => None
    };
}

fn distance_origin(p :Point) -> i32 {
    return i32::abs(p.x) + i32::abs(p.y);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Reading from file: {}", filename);
    let f = File::open(filename).expect("Could not open file");
    let file = BufReader::new(&f);

    let mut iter = file.lines();

    let s1 = iter.next();//.as_ref().unwrap().trim().split(",").map(parse_extension);
    let s2 = iter.next();//.unwrap().as_ref().unwrap();//.trim().split(",").map(parse_extension);

    let result = solve(&s1.unwrap().unwrap(), &s2.unwrap().unwrap());

    println!("Result: {}", result);

}

fn solve(s1 :&str, s2 :&str) -> i32 {
    let mut wire1 :Vec<Line>  = vec![];
    let mut wire2 :Vec<Line>  = vec![];
    let sw1 = s1.trim().split(",");
    let sw2 = s2.trim().split(",");
    sw1.map(parse_extension).for_each(|ext| extend_wire(&mut wire1, ext));
    sw2.map(parse_extension).for_each(|ext| extend_wire(&mut wire2, ext));

    let mut dist = std::i32::MAX;
    for l1 in &wire1 {
        for l2 in &wire2 {
            match get_intersection(l1, l2) {
                None => (),
                Some(d) => {
                    println!("dist: {}", d);
                    if d < dist {
                        dist = d;
                    }
                }
            }
        }
    }
    return dist;
}
#[cfg(test)]
mod tests {
    use crate::{solve};

    #[test]
    fn test() {
        /*
        //part 1
        assert_eq!(solve(&"R8,U5,L5,D3", &"U7,R6,D4,L4"), 6);
        assert_eq!(solve(&"R75,D30,R83,U83,L12,D49,R71,U7,L72", &"U62,R66,U55,R34,D71,R55,D58,R83"), 159);
        assert_eq!(solve(&"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", &"U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"), 135);
        */
        // Part 2
        assert_eq!(solve(&"R8,U5,L5,D3", &"U7,R6,D4,L4"), 30);
        assert_eq!(solve(&"R75,D30,R83,U83,L12,D49,R71,U7,L72", &"U62,R66,U55,R34,D71,R55,D58,R83"), 610);
        assert_eq!(solve(&"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", &"U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"), 410);
    }

}