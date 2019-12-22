use std::io::{BufReader, BufRead, stdout, Write};
use std::fs::File;
use regex::Regex;
use std::str::FromStr;

extern crate regex;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Action {
    DealIntoNewStack,
    Cut(i128),
    DealWithIncrement(i128),
}

// Encoded state first number + diff
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
struct Deck {
    first: i128,
    diff: i128,
    size: i128,
}


impl Deck {
    fn new(size :u128) -> Deck {
        return Deck {
            first: 0,
            diff: 1,
            size: size as i128,
        };
    }

    fn apply_instructions(&mut self, instructions :&Instructions) {
        for action in instructions {
            self.apply_action(action);
        }
    }

    // Reverse: *-1  diff*=-1
    // Cut(n): first + diff*n, diff
    // Take(n): self.diff = (self.diff * *n) % self.size; for primes..
    // Answer = (self.first * 2020 * self.diff)


    // Take 3:
    // Take 2020
    // Undo program n times,
    // Undo reverse -> (val = size - val)
    // Undo cut(n) -> (val += n)
    // Undo take(n) ->
    //

    fn apply_action(&mut self, action :&Action) {
        println!("Applying: {:?} to {:?} ({:?})", action, self, self.get_card_vec());
        match action {
            // Reverse: *-1  diff*=-1
            Action::DealIntoNewStack => {
                self.first += (self.size - 1) * self.diff;
                self.diff *= -1;
            },
            // Cut(n): first + diff*n, diff
            Action::Cut(n) => {
                self.first = (self.first + self.diff * *n) % self.size;
            },
            // Take(n): first, diff *= (size - n)
            Action::DealWithIncrement(n) => {
                // Find a/n for a = v+(10x)/n   for a % n == 0
                let mut v= self.diff;
                while v % n != 0 {
                    v += self.size;
                }
                self.diff =  v / n;
//                self.diff = (self.diff * *n) % self.size; //for primes..
            },
        }
    }

    fn get_card_vec(&self) -> Vec<i128> {
        let mut v = Vec::with_capacity(self.size as usize);
        let current = self.first;
        for i in 0..self.size {
            let val = (current + i*self.diff) % self.size;
            v.push( (val + self.size) % self.size);
        }
        return v;
    }
}

type Instructions = Vec<Action>;

fn parse_action(s :&str) -> Action {
    let dins_re: Regex = Regex::new(r"^deal into new stack$").unwrap();
    let cut_re: Regex = Regex::new(r"^cut\s+(-?\d+)$").unwrap();
    let dwi_re: Regex = Regex::new(r"^deal with increment\s+(-?\d+)$").unwrap();
    match dins_re.captures(s) {
        Some(m) => {
            return Action::DealIntoNewStack;
        }
        _ => {}
    }
    match cut_re.captures(s) {
        Some(m) => {
            let val = i128::from_str(&m[1]).unwrap();
            return Action::Cut(val)
        }
        _ => {}
    }
    match dwi_re.captures(s) {
        Some(m) => {
            let val = i128::from_str(&m[1]).unwrap();
            return Action::DealWithIncrement(val)
        }
        _ => {}
    }
    panic!("Could not parse string: {}", &s);
}

fn read_file(filename :&str) -> Instructions {
    let f = File::open(filename).expect("Could not open file");
    let file = BufReader::new(&f);
    let mut result = vec![];

    for line in file.lines() {
        let unwrapped = line.unwrap();
        let trimmed = unwrapped.trim();
        result.push(parse_action(&trimmed));
    }
    return result;
}

/*
Matrix reprpresentation
Reverse:
[-1  0] x [f]   =  [-1 * f]
[ 0 -1]   [d]      [-1 * d]

Cut(n):
[1  n ] x [f]   =  [f + n*d]
[0  1 ]   [d]      [d]

Take(n):
[1  0 ] x [f]   =  [f]
[0  (size-n)]   [d]      [d * (size - n)]
*/



fn main() {
//    let args: Vec<String> = env::args().collect();
//    let filename = &args[1];
    let filename = "data/day22/test5.txt";
    println!("Reading from file: {}", filename);

    let instructions = read_file(filename);
    let mut deck = Deck::new(97);
    //   let orig = deck.cards.clone();

    for i in 0..10 {
        deck.apply_instructions(&instructions);
        println!("cards: {:?}", deck.get_card_vec());
        //   if deck.cards == orig {
        //      println!("Applications: {}", i+1);
        // }
    }
//    deck.cards.iter().position(|x| *x == 2019).unwrap();
//    let pos = reverse_pos(2020, &instructions,119315717514047);

//   println!("Position: {}", pos);

//    println!("Result: {}", result);

}

#[cfg(test)]
mod tests {
    use crate::Action::{DealWithIncrement, DealIntoNewStack, Cut};
    use crate::{parse_action, Deck, read_file};

    #[test]
    fn test_parse_action() {
        assert_eq!(parse_action("deal with increment 52"), DealWithIncrement(52));
        assert_eq!(parse_action("cut -3134"), Cut(-3134));
        assert_eq!(parse_action("deal into new stack"), DealIntoNewStack);
    }

    #[test]
    fn test_deal_into_new_stack() {
        let mut deck = Deck::new(3);
        deck.apply_action(&DealIntoNewStack);
        assert_eq!(deck.get_card_vec(), [2,1,0]);
    }

    #[test]
    fn test_cut() {
        let mut deck = Deck::new(10);
        deck.apply_action(&Cut(3));
        assert_eq!(deck.get_card_vec(), [3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
    }

    #[test]
    fn test_cut_negative() {
        let mut deck = Deck::new(10);
        deck.apply_action(&Cut(-4));
        assert_eq!(deck.get_card_vec(), [6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_deal_with_increment() {
        let mut deck = Deck::new(10);
        deck.apply_action(&DealWithIncrement(3));
        assert_eq!(deck.get_card_vec(), [0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
    }

    #[test]
    fn test_deal_with_increment_7() {
        let mut deck = Deck::new(10);
        deck.apply_action(&DealWithIncrement(7));
        assert_eq!(deck.get_card_vec(), [0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
    }

    #[test]
    fn test_1() {
        let mut deck = Deck::new(10);
        let instructions = read_file("data/day22/test1.txt");
        deck.apply_instructions(&instructions);
        assert_eq!(deck.get_card_vec(), [0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
    }

    #[test]
    fn test_2() {
        let mut deck = Deck::new(10);
        let instructions = read_file("data/day22/test2.txt");
        deck.apply_instructions(&instructions);
        assert_eq!(deck.get_card_vec(), [3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
    }

    #[test]
    fn test_3() {
        let mut deck = Deck::new(10);
        let instructions = read_file("data/day22/test3.txt");
        deck.apply_instructions(&instructions);
        assert_eq!(deck.get_card_vec(), [6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
    }

    #[test]
    fn test_4() {
        let mut deck = Deck::new(10);
        let instructions = read_file("data/day22/test4.txt");
        deck.apply_instructions(&instructions);
        assert_eq!(deck.get_card_vec(), [9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    }

    #[test]
    fn test_part1() {
        let filename = "data/day22/input.txt";
        println!("Reading from file: {}", filename);
        let instructions = read_file(filename);
        let mut deck = Deck::new(10007);
        deck.apply_instructions(&instructions);
        let res = deck.get_card_vec();
        assert_eq!(res[1498], 2019);
    }


}