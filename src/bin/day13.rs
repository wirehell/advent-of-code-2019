use std::{env, thread};
use advent_of_code_2019::intmachine;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use std::sync::mpsc::{SyncSender, Receiver};
use advent_of_code_2019::intmachine::{Message, Word};
use std::sync::mpsc;
use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use crate::Tile::{Empty, Wall, Block, Paddle, Ball};
use crate::ArcadeState::{ReadY, ReadTile, ReadX};


#[derive(Clone, Eq, PartialEq, Debug)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}
impl Tile {
    fn from(v :Word) -> Tile {
        return match v {
            0 => Empty,
            1 => Wall,
            2 => Block,
            3 => Paddle,
            4 => Ball,
            _ => panic!("Unknown tile"),
        }
    }
    fn repr(&self) -> char {
        return match self {
            Empty => '.',
            Wall => '#',
            Block => '%',
            Tile::Paddle => '-',
            Tile::Ball => 'O',
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Screen {
    x_size :i64,
    y_size :i64,
    pixels: Vec<Tile>
}

impl Screen {
    fn new(x_size :i64, y_size :i64) -> Screen {
        Screen {
            x_size,
            y_size,
            pixels: vec![Empty; (x_size * y_size) as usize]
        }
    }

    fn draw(&mut self, tile: Tile, x :&Word, y :&Word) {
        assert!(*x < self.x_size);
        assert!(*y < self.y_size);
        self.pixels[(y*self.x_size + x) as usize] = tile;
    }

    fn print(&self) {
        for y in 0..self.y_size {
            for x in 0..self.x_size {
                let element = self.pixels[(y*self.x_size + x) as usize].borrow();
                print!("{}", element.repr());
            }
            println!("");
        }
    }

    fn count_block_tiles(&self) -> usize {
        return self.pixels.iter()
            .filter(|t| *t.clone() == Block)
            .count();
    }
}

enum ArcadeState {
    ReadX,
    ReadY(Word),
    ReadTile(Word, Word),
}

struct ArcadeCabinet {
    screen :Screen,
    score :Word,
    state :ArcadeState,
}

impl ArcadeCabinet {
    fn new() -> ArcadeCabinet {
        let screen = Screen::new(50, 25);
        return ArcadeCabinet {
            screen,
            state: ArcadeState::ReadX,
            score :0,
        }
    }

    fn output(&mut self, v :Word) {
        match self.state {
            ArcadeState::ReadX => {
                self.state = ReadY(v)
            }
            ArcadeState::ReadY(x) => {
                self.state = ReadTile(x, v)
            }
            ArcadeState::ReadTile(x, y) => {
                if x == -1 && y == 0 {
                    self.score = v;
                } else {
                    self.screen.draw(Tile::from(v), &x, &y);
                }
                self.state = ReadX;
            }
        }
    }

    fn print(&self) {
        self.screen.print();
    }

}




fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut program = intmachine::read_program(filename);
//    program[0] = 2; // Free to play hack!!

    let mut arcade = ArcadeCabinet::new();

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
                        arcade.output(data)
                    }

                    Message::Shutdown => break,
                    Message::RequestInput => {
                        panic!("Unexpected input");
                    }
                }
            }
            Err(error) => {
                panic!("Error: {:?}", error.to_string());
            }
        }
    }
    child.join();

    let result = arcade.screen.count_block_tiles();
    arcade.screen.print();
    println!("Result: {}", result);

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