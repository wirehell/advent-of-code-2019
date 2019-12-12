use std::{env, thread};
use advent_of_code_2019::intmachine;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use std::sync::mpsc::{SyncSender, Receiver};
use advent_of_code_2019::intmachine::Message;
use std::sync::mpsc;
use std::borrow::Borrow;
use std::collections::hash_map::RandomState;

type Pos = (i32, i32);

#[derive(Clone, Eq, PartialEq, Debug)]
enum Color {
    White, Black,
}

type HullPaint = Vec<Vec<Color>>;

#[derive(Clone, Eq, PartialEq, Debug)]
struct Hull {
    paint: Rc<RefCell<HullPaint>>,
}

const HULL_SIZE_X :i32= 100;
const HULL_SIZE_Y :i32= 100;

impl Hull {
    fn new() -> Hull {
        let mut h = vec![];
        for y in 0..HULL_SIZE_Y {
            h.push(vec![Color::Black; HULL_SIZE_X as usize]);
        }
        h[50][50] = Color::White;
        return Hull {
            paint: Rc::new(RefCell::new(h))
        }
    }

    fn paint(&self, pos :Pos, color :Color) {
        println!("Painting {:?} with {:?}", pos, color);
        let mut hull = self.paint.borrow_mut();
        let (x, y) = pos;
        hull[y as usize][x as usize] = color;
    }

    fn print(&self) {
 //       let hull: &RefCell<HullPaint> = self.paint.borrow();
//        let h = hull.borrow();
        let mut hull = self.paint.borrow_mut();
        for y in 0..HULL_SIZE_Y {
            for x in 0..HULL_SIZE_X {
                match hull[y as usize][x as usize] {
                    Color::Black => print!("."),
                    Color::White => print!("#"),
                }
            }
            println!("");
        }

        println!("size: {}", hull.len())
    }

    fn get_color(&self, pos :Pos) -> Color {
        let hull: &RefCell<HullPaint> = self.paint.borrow();
        let h = hull.borrow();
        let (x, y) = pos;
        return h[y as usize][x as usize].clone();
    }

}


#[derive(Clone, Eq, PartialEq, Debug)]
enum State {
    Turning,
    Painting
}

type Command = i64;

#[derive(Clone, Eq, PartialEq, Debug)]
enum Direction {
    Up, Down, Left, Right
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct PaintBot {
    direction: Direction,
    state: State,
    pos: (i32, i32),
    hull: Rc<Hull>,
}

impl PaintBot {
    pub fn new(hull :&Rc<Hull>) -> PaintBot {
        return PaintBot {
            direction: Direction::Up,
            state: State::Painting,
            pos : (50, 50),
            hull : hull.clone(),
        }
    }

    fn turn(&mut self, command :&Command) {
        let new_dir = match command {
            // ccw
            0 => {
                match self.direction {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                }
            }
            1 => {
                match self.direction {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                }

            }
            _ =>  unreachable!() ,
        };
        println!("Turned to: {:?}", new_dir);
        self.direction = new_dir;
    }
    fn paint(&mut self, command :&Command) {
        let new_color = match command {
            0 => Color::Black,
            1 => Color::White,
            _ => unreachable!(),
        };
        self.hull.paint(self.pos, new_color);
    }

    fn step(&mut self) {
        let (x, y) = self.pos;
        let new_pos = match self.direction {
            Direction::Up => (x, y - 1),
            Direction::Down =>  (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        };
        println!("Moving to: {:?}", &new_pos);
        self.pos = new_pos;

    }

    pub fn execute(&mut self, command: &Command) {
        match self.state {
            State::Turning => {
                self.turn(command);
                self.state = State::Painting;
                self.step();
            }
            State::Painting => {
                self.paint(command);
                self.state = State::Turning;
            }
        }
    }

    pub fn scan(&self) -> Color {
        return self.hull.get_color(self.pos);
    }

}



fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let program = intmachine::read_program(filename);

    let hull = Rc::new(Hull::new());
    let mut bot = PaintBot::new(&hull);

    let (input, pin): (SyncSender<Message>, Receiver<Message>) = mpsc::sync_channel(0);
    let (pout, output): (SyncSender<Message>, Receiver<Message>) = mpsc::sync_channel(0);

    let child = thread::spawn(move || {
        intmachine::execute(&program, pin, pout);
    });


    loop {
        match output.recv() {
            Ok(message) => {
                match message {
                    Message::Data(data) =>  {
                        bot.execute(&data);
                    }

                    Message::Shutdown => break,
                    Message::RequestInput => {
                        let scan = match bot.scan() {
                            Color::Black => 0,
                            Color::White => 1,
                        };
                        input.send(Message::Data(scan));

                    }
                }
            }
            Err(error) => {
                panic!("Error: {:?}", error.to_string());
            }
        }
    }
    child.join();

    hull.print();
}

#[cfg(test)]
mod tests {

    /*
    #[test]
    fn test() {
        assert_eq!(split_and_parse(&"1,2,3"), [1,2,3])
    }
    */

}