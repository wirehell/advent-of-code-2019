use std::{thread};
use advent_of_code_2019::intmachine;
use std::collections::{HashMap, VecDeque};
use std::sync::mpsc::{SyncSender, Receiver, Sender, RecvTimeoutError, RecvError};
use advent_of_code_2019::intmachine::{Message, Word, IO, StandardIO};
use std::sync::mpsc;
use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;
use std::io::BufRead;


fn main() {

    let filename = "data/day25/input.txt";
    let mut program = intmachine::read_program(filename);

    let (console_in, stdin) : (SyncSender<Message>, Receiver<Message>) = mpsc::sync_channel(0);
    let (stdout, console_out): (SyncSender<Message>, Receiver<Message>) = mpsc::sync_channel(0);

    let child = thread::spawn(move || {
        let mut io = StandardIO { stdin, stdout };
        intmachine::execute(&program, &mut io);
    });
    let output = thread::spawn(move || {
        loop {
            match console_out.recv() {
                Ok(Message::Data(data)) => {
                    print!("{}", (data as u8) as char);
                },
                Ok(Message::RequestInput) => {

                },
                _ => {
                    panic!("Unhandled output");
                },
            }
        }
    });

    loop {
        for line in std::io::stdin().lock().lines() {
            for c in line.unwrap().chars() {
                console_in.send(Message::Data(c as Word));
            }
            console_in.send(Message::Data('\n' as Word));
        }
    }

    child.join();
    output.join();

}

#[cfg(test)]
mod tests {

}