use std::io::{BufReader, BufRead, stdout, Write};
use std::fs::File;
use regex::Regex;
use std::str::FromStr;
use num::{Integer, BigInt, ToPrimitive, Zero, One, Signed};
use std::borrow::Borrow;
use std::ops::BitAnd;

extern crate regex;

type Size = BigInt;
type Pos = BigInt;

#[derive(Clone, Eq, PartialEq, Debug)]
enum Action {
    DealIntoNewStack,
    Cut(Pos),
    DealWithIncrement(Pos),
}

// Encoded state first number + diff
#[derive(Clone, Eq, PartialEq, Debug)]
struct Deck {
    first: Pos,
    diff: Pos,
    size: Size,
}

impl Deck {
    fn new(size :u128) -> Deck {
        return Deck {
            first: Pos::from(0),
            diff: Pos::from(1),
            size: Size::from(size),
        };
    }

    fn apply_instructions(&mut self, instructions :&Instructions) {
        for action in instructions {
            self.apply_action(action);
        }
    }

    fn apply_action(&mut self, action :&Action) {
//        println!("Applying: {:?} to {:?} ({:?})", action, self, self.get_card_vec());
        match action {
            // Reverse: *-1  diff*=-1
            Action::DealIntoNewStack => {
                self.first += (self.size.borrow() - 1) * self.diff.borrow();
                self.diff *= -1;
            },
            // Cut(n): first + diff*n, diff
            Action::Cut(n) => {
                self.first = (self.first.borrow() + self.diff.borrow() * n.borrow()) % &self.size;
            },
            // Take(n): first, diff *= (size - n)
            Action::DealWithIncrement(n) => {
                // Find a/n for a = v+(10x)/n   for a % n == 0
                let mut v= self.diff.clone();
                while &v % n != Pos::from(0) {
                    v += &self.size;
                }
                self.diff =  v / n;
//                self.diff = (self.diff * *n) % self.size; //for primes..
            },
        }
    }

    fn get_card_vec(&self) -> Vec<i128> {
        let size = self.size.borrow();
        let mut v :Vec<i128> = Vec::with_capacity(size.to_usize().unwrap());
        let current = self.first.to_i128().unwrap();
        for i in 0..self.size.to_usize().unwrap() {
            let val = (current + (i as i128) * self.diff.to_i128().unwrap()) % size.to_i128().unwrap();
            v.push( (val + size.to_i128().unwrap()) % size.to_i128().unwrap());
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
            let val = Pos::from_str(&m[1]).unwrap();
            return Action::Cut(val)
        }
        _ => {}
    }
    match dwi_re.captures(s) {
        Some(m) => {
            let val = Pos::from_str(&m[1]).unwrap();
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

// Take 3: reverse..
// Undo program n times,
// Undo reverse -> (val = size - 1 - val)
// Undo cut_n(val) -> (val = (val + n) mod)
// Undo take_n,val -> (n^(97-2) * val) mod 97
//
fn reverse_one(pos: &Pos, action :&Action, size: &Size) -> Pos {
    match action {
        Action::DealIntoNewStack => {
            return size - 1 - pos;
        },
        Action::Cut(n) => {
            return (size + pos + n) % size;
        },
        Action::DealWithIncrement(n) => {
            return (pos.clone() * n.modpow(&(size - 2), &size)).mod_floor(size);
        },
    }
}

// All transformations is on form x(k) = a*x(k-1) + b
type Transform = (BigInt, BigInt);

fn extract_transform(instructions: &Instructions, size: &BigInt) -> Transform {
    let mut transform: Transform = (BigInt::one(), BigInt::zero());
    for i in instructions.iter() {
//        println!("Transform: {:?} for: {:?}", transform, i);
        match i {
            // Where x(k) = (a, b) (ax + b)
            Action::DealIntoNewStack => {
                // x(k) = -x(k-1) + (1 - size)
                let a = -1 * transform.0.borrow();
                let b = -1 * transform.1.borrow() + size.borrow() - BigInt::one();
                transform = (a, b);
            },
            Action::Cut(n) => {
                // x(k) = x(k-1) - n
                let a = transform.0.borrow();
                let b = transform.1.borrow() - n;
                transform = (a.clone(), b);
            },
            Action::DealWithIncrement(n) => {
                // x(k) = n*x(k-1)
                let a = transform.0.borrow() * n;
                let b = transform.1.borrow() * n;
                transform = (a, b);
            },
        }
    }
//    println!("Returning transform: {:?}", transform);
    return transform;
}


fn repeat_transform(transform: &Transform, n: &BigInt, size: &BigInt) -> Transform {
    // Adjust to positive numbers.
    let adj_a :BigInt = (transform.0.borrow() / size.borrow() + BigInt::one()).abs() * size.borrow();
    let adj_b :BigInt = (transform.0.borrow() / size.borrow() + BigInt::one()).abs() * size.borrow();

    let a = transform.0.borrow() + adj_a;
    let b = transform.1.borrow() + adj_b;
    /*
    Repeated ^n by continuous substitution which yields;
    x(k) ≅ a^n * x(0) + b + b*a + b*a^2 + .. + b*a^(n-1) . This is a geomeric series, for a!=0
    x(k) ≅ a^n * x(0) + b*(a^n-1) * 1/(a-1)
    if a=1 and n !=0
    x(k) ≅ a^n * x(0) + b + b + b + .. + b . This

    Fermats little theorem says that x^(n-1) ≅ 1 (mod n if n is prime), insert (1-a) as x
    x(k) ≅ a^n * x(0) + b*(a^n-1) * (a-1)^(size-1)/(a-1)
    x(k) ≅ a^n * x(0) + b*(a^n-1) * (a-1)^(size-2)
           ----         --------------------------
           repeated_a         repeated_b
    */

    let repeated_a;
    let repeated_b;

    repeated_a = a.modpow(n.borrow(), size.borrow());
    repeated_b = b.borrow() * &(a.modpow(&n.borrow() , size.borrow()) - BigInt::one().borrow()) *
        (a - BigInt::one()).modpow(&(size.borrow() - BigInt::from(2)), size.borrow());

    return (repeated_a, repeated_b);

}

fn reverse(pos: &BigInt, n: &BigInt, size: &BigInt, transform: &Transform) -> BigInt {

    // Now we solve x(k) ≅ a*x(0) + b (mod size)
    // This has the solution x(0) = a^(size-2) * (x(k) - b)
    let (repeated_a, repeated_b) = repeat_transform(&transform, &n, size.borrow());
    let x0 = (repeated_a.modpow(&(size.borrow() - BigInt::from(2)), size.borrow()) * (pos - repeated_b));//.mod_floor(size.borrow());
    return x0;
}


fn simple_reverse(pos: &Pos, instructions: &Instructions, size: &Size) -> Pos {
    let mut res :Pos = pos.clone();
    for i in instructions.iter().rev() {
        res = reverse_one(&res, i, &size);
    }
    return res;
}



fn main() {
    let filename = "data/day22/input.txt";
    println!("Reading from file: {}", filename);

    let instructions = read_file(filename);
//    let deck = Deck::new(97);
    //   let orig = deck.cards.clone();

    let size = BigInt::from(119315717514047i128);
    let shuffles = BigInt::from(101741582076661i128);

    let transform = extract_transform(&instructions, &size);

    let result = reverse(&BigInt::from(2020), &shuffles, &size, &transform).mod_floor(&size);
    println!("Result: {}", result.to_i128().unwrap());

//   println!("Position: {}", pos);

//    println!("Result: {}", result);

}

#[cfg(test)]
mod tests {
    use crate::Action::{DealWithIncrement, DealIntoNewStack, Cut};
    use crate::{parse_action, Deck, read_file, reverse_one, simple_reverse, Pos, Size, extract_transform, reverse, repeat_transform};
    use num::{BigInt, ToPrimitive, One, Zero, Integer};
    use std::borrow::Borrow;


    #[test]
    fn test_parse_action() {
        assert_eq!(parse_action("deal with increment 52"), DealWithIncrement(Pos::from(52)));
        assert_eq!(parse_action("cut -3134"), Cut(Pos::from(-3134)));
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
        deck.apply_action(&Cut(Pos::from(3)));
        assert_eq!(deck.get_card_vec(), [3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
    }

    #[test]
    fn test_reverse_cut() {
        let mut deck = Deck::new(97);
        deck.apply_action(&Cut(Pos::from(3)));
        let actions = vec![Cut(Pos::from(3))];
        let transform = extract_transform(&actions, &Pos::from(97));
        let v = deck.get_card_vec();
        println!("{:?}", v);
        for x in 0..v.len()  {
            println!("{:?}", x);
            assert_eq!(deck.get_card_vec()[x], reverse_one(&Pos::from(x), &Cut(Pos::from(3)), &Size::from(97)).to_i128().unwrap());
            let rev = reverse(&Pos::from(x), &BigInt::from(1), &BigInt::from(97), &transform);
            println!("Rev: {:?}", rev);
            assert_eq!(deck.get_card_vec()[x], rev.to_i128().unwrap());
        }
    }

    #[test]
    fn test_cut_negative() {
        let mut deck = Deck::new(10);
        deck.apply_action(&Cut(Pos::from(-4)));
        assert_eq!(deck.get_card_vec(), [6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_cut_negative() {
        let mut deck = Deck::new(97);
        deck.apply_action(&Cut(Pos::from(-3)));
        let actions = vec![Cut(Pos::from(-3))];
        let transform = extract_transform(&actions, &Pos::from(97));
        let v = deck.get_card_vec();
        println!("{:?}", v);
        for x in 0..v.len() {
            assert_eq!(deck.get_card_vec()[x], reverse_one(&Pos::from(x), &Cut(Pos::from(-3)), &Size::from(97)).to_i128().unwrap());
            let rev = reverse(&Pos::from(x), &BigInt::from(1), &BigInt::from(97), &transform);
            assert_eq!(deck.get_card_vec()[x], rev.to_i128().unwrap());
        }
    }

    #[test]
    fn test_deal_with_increment() {
        let mut deck = Deck::new(10);
        deck.apply_action(&DealWithIncrement(Pos::from(3)));
        assert_eq!(deck.get_card_vec(), [0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
    }

    #[test]
    fn test_reverse_deal_with_increment() {
        let mut deck = Deck::new(97);
        deck.apply_action(&DealWithIncrement(Pos::from(10)));
        let actions = vec![DealWithIncrement(Pos::from(10))];
        let transform = extract_transform(&actions, &Pos::from(97));
        let v = deck.get_card_vec();
        println!("{:?}", v);
        for x in 0..v.len() {
            println!("{:?}", x);
            assert_eq!(deck.get_card_vec()[x], reverse_one(&Pos::from(x), &DealWithIncrement(Pos::from(10)), &Size::from(97)).to_i128().unwrap());
            let rev = reverse(&Pos::from(x), &BigInt::from(1), &BigInt::from(97), &transform);
            println!("REv: {:?}", rev);
            assert_eq!(deck.get_card_vec()[x], rev.to_i128().unwrap());
        }
    }

    #[test]
    fn test_deal_with_increment_7() {
        let mut deck = Deck::new(10);
        deck.apply_action(&DealWithIncrement(Pos::from(7)));
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
        let instructions = read_file(filename);
        let mut deck = Deck::new(10007);
        deck.apply_instructions(&instructions);
        deck.apply_instructions(&instructions);
        deck.apply_instructions(&instructions);
        let res = deck.get_card_vec();
//        assert_eq!(res[1498], 2019);

        let size = Pos::from(10007);
        let transform = extract_transform(&instructions, &Pos::from(10007));
        let v = deck.get_card_vec();
        println!("{:?}", v);
        for x in 0..v.len() {
            println!("WOO{:?}", x);
            let rev = reverse(&Pos::from(x), &BigInt::from(3), &BigInt::from(10007), &transform);
            //println!("REv: {:?}", rev);
            assert_eq!(deck.get_card_vec()[x], rev.mod_floor(&size).to_i128().unwrap());
        }
    }

    #[test]
    fn test_simple_reverse() {
        let filename = "data/day22/input.txt";
        let instructions = read_file(filename);
        let pos = simple_reverse(&Pos::from(1498), &instructions, &Size::from(10007));
        assert_eq!(pos, Pos::from(2019));
    }
    #[test]
    fn test_reverse() {
        let filename = "data/day22/input.txt";
        let instructions = read_file(filename);
        let transform = extract_transform(&instructions, &Size::from(10007));
        println!("Transform is: {:?}", transform);
        let res = reverse(&Pos::from(1498), &BigInt::one(), &Size::from(10007), &transform);
        println!("Res: {:?}", res);
    }

    #[test]
    fn test_extract_transform() {
        let filename = "data/day22/input.txt";
        let size = Size::from(10007);
        let instructions = read_file(filename);
        let transform = extract_transform(&instructions, &size);
        println!("Transform is: {:?}", transform);
        let res = (Pos::from(2019) * transform.0.borrow() + transform.1.borrow()).mod_floor(&size);
        assert_eq!(res, Pos::from(1498));
    }

    #[test]
    fn test_transform_repeat() {
        let size = Size::from(97);
        let instructions = vec![
            DealWithIncrement(Pos::from(10)),
            Cut(Pos::from(3)),
            DealIntoNewStack,
        ];
        let n = BigInt::from(13);
        let mut ri = vec![];
        for i in 0..n.to_u32().unwrap() {
            ri.extend(instructions.clone())
        }

        let pos = BigInt::from(7);
        // Repeated by instructions
        let repeated_transform = extract_transform(&ri, &size);
        println!("repeated: {:?}", &repeated_transform);

        // Repeated by ^n
        let single_transform = extract_transform(&instructions, &size);
        println!("single: {:?}", &single_transform);


        let rt = repeat_transform(&single_transform, &n, &size);

        println!("r1: {:?} r2: {:?}", repeated_transform, rt);

        let a = (pos.borrow() * repeated_transform.0.borrow() + repeated_transform.1.borrow()).mod_floor(size.borrow());
        let b = (pos.borrow() * rt.0.borrow() + rt.1.borrow()).mod_floor(size.borrow());

        assert_eq!(a,b);
    }
}
