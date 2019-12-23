use std::{thread};
use advent_of_code_2019::intmachine;
use std::collections::{HashMap, VecDeque};
use std::sync::mpsc::{SyncSender, Receiver, Sender, RecvTimeoutError};
use advent_of_code_2019::intmachine::{Message, Word, IO};
use std::sync::mpsc;
use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;
use crate::NicReceiveState::{ReadY, Nothing};
use crate::NicSendState::ReadAddress;

#[derive(Debug,Clone)]
struct DataPacket {
    source: Word,
    dest: Word,
    x: Word,
    y: Word,
}

#[derive(Debug,Clone)]
enum NetworkPacket {
    Data(DataPacket),
    Idle(Word),
}


enum NicSendState {
    ReadAddress,
    ReadX(Word),
    ReadY(Word,Word),
}

enum NicReceiveState {
    Nothing,
    ReadY(Word),
}

struct NetworkIO {
    network_send: Sender<NetworkPacket>,
    network_receive: Receiver<NetworkPacket>,
    send_state: NicSendState,
    receive_state: NicReceiveState,
    address: Word,
    initialized : bool, // Remove when read
    idle_counter: i64,
}

impl IO for NetworkIO {
    fn send(&mut self, message: Message) -> () {
    //    println!("{:?}", message);
        let m = match message {
            Message::Shutdown => {
                panic!()
            },
            Message::Data(x) => {
                x
            },
            Message::RequestInput => {
                return;
            },
        };

        self.send_state = match self.send_state {
            NicSendState::ReadAddress => {
                NicSendState::ReadX(m)
            },
            NicSendState::ReadX(address) => {
                NicSendState::ReadY(address, m)
            },
            NicSendState::ReadY(address, x) => {
                self.idle_counter = 0;
                self.network_send.send(NetworkPacket::Data(DataPacket {
                    dest: address,
                    x,
                    y: m,
                    source: self.address,
                }));
                NicSendState::ReadAddress
            },
        }
    }

    fn receive(&mut self) -> Message {
        if !self.initialized {
            println!("Set adddress: {}", self.address);
            self.initialized = true;
            return Message::Data(self.address);
        }

        match self.receive_state {
            NicReceiveState::Nothing => {
                match self.network_receive.recv_timeout(Duration::from_millis(0)) {
                    Ok(NetworkPacket::Data(p)) => {
                        self.idle_counter = 0;
                        self.receive_state = ReadY(p.y);
                        return Message::Data(p.x);
                    },
                    Err(RecvTimeoutError::Timeout) => {
                        self.idle_counter+=1;
                        if self.idle_counter == 10000 {
                            self.network_send.send(NetworkPacket::Idle(self.address));
                        }
                        return Message::Data(-1); // No data available
                    },
                    x => { panic!(x)}
                }

            },
            NicReceiveState::ReadY(y) => {
                self.receive_state = Nothing;
                return Message::Data(y);
            },
        }
    }
}



fn main() {

    let filename = "data/day23/input.txt";
    let mut program = intmachine::read_program(filename);

    let mut children = vec![];
    let mut network_interfaces = vec![];
    let mut idle_state = vec![false ;50];

    let (nic_send, network_input) : (Sender<NetworkPacket>, Receiver<NetworkPacket>) = mpsc::channel();

    for i in 0..50 {
        let (network_output, nic_receive): (Sender<NetworkPacket>, Receiver<NetworkPacket>) = mpsc::channel();

        let mut network_io = NetworkIO {
            network_send: nic_send.clone(),
            network_receive: nic_receive,
            send_state: NicSendState::ReadAddress,
            receive_state: NicReceiveState::Nothing,
            address: i,
            initialized: false,
            idle_counter: 0,
        };


        let mem = program.clone();
        let child = thread::spawn(move || {
            intmachine::execute(&mem, &mut network_io);
        });

        network_interfaces.push(network_output);
        children.push(child);
    }

    let mut nat = None;
    let mut last = None;

    loop {
        match network_input.recv() {
            Ok(NetworkPacket::Data(message)) => {
                let dest = message.dest;
                if dest == 255 {
                    println!("Nat received: {:?}", &message);
                    nat = Some(message);
//                    break;
                } else {
                    idle_state[message.source as usize] = false;
                    idle_state[message.dest as usize] = false;
//                    println!("{:?}", message);
                    network_interfaces[dest as usize].send(NetworkPacket::Data(message)).unwrap();
                }
            }
            Ok(NetworkPacket::Idle(source)) => {
//                println!("Setting: {} as idle", source);
                idle_state[source as usize] = true;
                let ic = idle_state.iter().cloned().filter(|x| *x).count();

                if ic == 50 {
                    let mut m = nat.clone().unwrap();
                    m.dest = 0;
                    println!("Sending NAT to [0]");
                    match last {
                        None => {},
                        Some(y) => {
                            if y  == m.y {
                                println!("Found solution: {}", y);
                                println!("{:?}", m);
                            }

                        },
                    }
                    last = Some(m.y);
                    network_interfaces[0].send(NetworkPacket::Data(m)).unwrap();
                }
            }

            Err(error) => {
                panic!("Error: {:?}", error.to_string());
            }
        }
    }

    for child in children {
        child.join();
    }

}

#[cfg(test)]
mod tests {

}