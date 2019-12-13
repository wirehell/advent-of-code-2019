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
use std::thread::sleep;
use std::time::Duration;


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
    pixels: Vec<Tile>,
    paddle_pos: (Word, Word),
    ball_pos: (Word, Word),
}

impl Screen {
    fn new(x_size :i64, y_size :i64) -> Screen {
        Screen {
            x_size,
            y_size,
            pixels: vec![Empty; (x_size * y_size) as usize],
            paddle_pos: (0, 0),
            ball_pos: (0, 0)
        }
    }

    fn draw(&mut self, tile: Tile, x :&Word, y :&Word) {
        assert!(*x < self.x_size);
        assert!(*y < self.y_size);
        match tile {
            Paddle => self.paddle_pos = (*x, *y),
            Ball => self.ball_pos = (*x, *y),
            _ => {},
        }
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

fn calc_input(screen :&Screen) -> Word {
//    print!("{}[2J", 27 as char);
//    screen.print();
//    sleep(Duration::from_millis(100));
    if screen.ball_pos.0 > screen.paddle_pos.0 {
        return 1;
    } else if screen.ball_pos.0 < screen.paddle_pos.0 {
        return -1
    } else {
        return 0;
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut program = intmachine::read_program(filename);
    program[0] = 2; // Free to play hack!!

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
                        arcade.output(data);

                    }

                    Message::Shutdown => break,
                    Message::RequestInput => {
                        input.send(Message::Data(calc_input(&arcade.screen)));
                    }
                }
            }
            Err(error) => {
                panic!("Error: {:?}", error.to_string());
            }
        }
    }
    child.join();

    let result = arcade.score;
    arcade.screen.print();
    println!("Score: {}", result);

}

#[cfg(test)]
mod tests {

}