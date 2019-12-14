use std::io::{BufReader, BufRead};
use std::fs::File;

use std::collections::{HashMap};
use std::str::FromStr;

extern crate regex;

type Name = String;
type Amount = i64;

type SubstanceAmount = (Name, Amount);

#[derive(Debug)]
struct NanoFactory {
    storage:  HashMap<String, i64>,
    ore: i64,
    formulas: HashMap<String, Formula>,
}

fn ceil_div(x: &i64, y:&i64) -> i64 {
    return x/y + if x % y != 0 { 1 } else { 0 }
}

impl NanoFactory {
    fn new(formulas :&Vec<Formula>) -> NanoFactory {
        let mut formula_map :HashMap<String, Formula> = HashMap::new();
        for formula in formulas {
            let (name, _a) = &formula.1;
            formula_map.insert(String::from(name), (*formula).clone());
        }
        return NanoFactory {
            storage: HashMap::new(),
            ore: 0,
            formulas: formula_map,
        }
    }

    fn get(&mut self, subst: &SubstanceAmount)  {
        //println!("Consuming: {:?}", subst);
        let (name, amount) = subst;
        if name == "ORE" {
            self.ore += *amount;
            return;
        }
        let missing;
        {
            let mut s = self.storage.entry(name.clone()).or_insert(0);
            //println!("Available of {} : {}", name, s);
            *s -= *amount;

            //println!("Left after get {} : {}", &name, *s);
            if *s < 0 {
                missing = Some((*s).abs());
            } else {
                missing = None;
            }
        }
        //println!("Missing in storage: {:?}", &missing);
        match missing {
            Some(num) => self.produce(&(name.clone(), num)),
            None => {}
        }
    }

    fn produce(&mut self, subst :&SubstanceAmount) {
        let (name, amount) = subst;
        //println!("Producing: {:?}", subst);
        let order;
        let produced_amount;
        {
            let formula :&Formula = self.formulas.get(name).unwrap();
            let o = create_order(formula, amount);
            //println!("Order: {:?}", &o);
            order = o.0;
            produced_amount = o.1;
        }
        {
            let mut entry = self.storage.entry(name.clone()).or_insert(0);
            *entry = *entry + produced_amount;
            //println!("New amount of {}: {}", &name, entry);
        }
        // Borrowed item creation
        {
            for item in order {
                let (name, amount) = &item;
                self.get(&item);
            }
        }
    }

    fn get_ore_used_(&self) -> i64 {
        return self.ore;
    }
}

fn create_order(formula :&Formula, amount :&i64) -> (Vec<SubstanceAmount>, i64) {
    let mut order = vec![];
    let (ingredients, output) = formula;
    let (_o, output_amount) = output;
    let times = ceil_div(amount , output_amount);
    for chemical in ingredients {
        let (n, a) = chemical;
        order.push( (n.clone(), *a * times))
    }
    return (order, output_amount * times);
}

type Ingredients = Vec<SubstanceAmount>;
type Formula = (Ingredients, SubstanceAmount);


fn parse_substance(s: &str) -> SubstanceAmount {
    let parts :Vec<&str> = s.trim().split(' ').collect();
    let amount = i64::from_str(parts[0]).unwrap();
    let name = String::from(parts[1]);
    return (name, amount)
}

fn parse_line(line: &str) -> Formula {
    let parts :Vec<&str> = line.split("=>").collect();
    let input = parts[0];
    let output = parts[1];
    let ingredients_strings = input.split(",");
    let mut ingredients = vec![];
    for s in ingredients_strings {
        ingredients.push(parse_substance(s));
    }
    let out = parse_substance(output);
    return (ingredients, out);
}

fn read_file(filename :&str) -> Vec<Formula> {

    let f = File::open(filename).expect("Could not open file");
    let file = BufReader::new(&f);

    let mut formulas = vec![];
    for line in file.lines() {
       formulas.push(parse_line(&line.unwrap()));
    }

    return formulas;
}

fn main() {
    /*
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    */
    let filename = "data/day14/input.txt";

    let formulas  = read_file(filename);
    let mut factory = NanoFactory::new(&formulas);
    factory.get(&(String::from("FUEL"), 1));

    println!("{:?}", &factory);
    println!("{:?}", &factory.ore);

    println!("Result: {}", fuel_per_trillion_ore(&formulas, 0, 10000000000000));
}


fn evalute(formulas: &Vec<Formula>, fuel: i64) -> i64 {
    let mut factory = NanoFactory::new(&formulas);
    factory.get(&(String::from("FUEL"), fuel));
    return factory.get_ore_used_();

}



fn fuel_per_trillion_ore(formulas: &Vec<Formula>, lo: i64, hi: i64) -> i64 {
    let target = 1000000000000;

    if hi < lo {
        println!("lo: {}  hi:{}", &lo, &hi);
        return hi;
    }

    let middle = (hi + lo) / 2;
    let val = evalute(formulas, middle);

    println!("lo: {} hi: {} mid: {} val: {}", &lo, &hi, &middle, &val);

    if val > target {
        return fuel_per_trillion_ore(formulas, lo, middle - 1);
    } else if val < target {
        return fuel_per_trillion_ore(formulas, middle + 1, hi);
    } else {
        println!("WFT {}", &middle);
        return middle
    }
}

#[cfg(test)]
mod tests {
    use crate::{read_file, parse_line, NanoFactory, ceil_div, fuel_per_trillion_ore};

    #[test]
    fn test_ceil_div() {
        assert_eq!(ceil_div(&10, &5), 2);
        assert_eq!(ceil_div(&10, &6), 2);
        assert_eq!(ceil_div(&10, &10), 1);
    }

    #[test]
    fn test_1() {
        let formulas = read_file("./data/day14/test1.txt");
        let mut factory = NanoFactory::new(&formulas);
        factory.get(&(String::from("FUEL"), 1));
        assert_eq!(factory.get_ore_used_(), 31);
    }

    #[test]
    fn test_13312() {
        let formulas = read_file("./data/day14/test3.txt");
        let mut factory = NanoFactory::new(&formulas);
        factory.get(&(String::from("FUEL"), 1));
        assert_eq!(factory.get_ore_used_(), 13312);
    }

    #[test]
    fn test_trillion_13312() {
        let formulas = read_file("./data/day14/test3.txt");
        assert_eq!(fuel_per_trillion_ore(&formulas, 0, 1000000000000), 82892753);
    }

    #[test]
    fn test_trillion_180697() {
        let formulas = read_file("./data/day14/test4.txt");
        assert_eq!(fuel_per_trillion_ore(&formulas, 0, 1000000000), 5586022);
    }

    #[test]
    fn test_parse_short() {
        let input = "7 CXHK => 1 FUEL";
        assert_eq!(parse_line(input),
                   (
                       vec![
                           (String::from("CXHK"), 7),
                       ] ,
                       (String::from("FUEL"), 1))
        );

    }

    #[test]
    fn test_parse_long_line() {
        let input = "7 CXHK, 2 XTMRV, 6 WSNPZ, 12 LQXCP => 1 FUEL";
        assert_eq!(parse_line(input),
                   (
                       vec![
                           (String::from("CXHK"), 7),
                           (String::from("XTMRV"), 2),
                           (String::from("WSNPZ"), 6),
                           (String::from("LQXCP"), 12),
                       ] ,
                       (String::from("FUEL"), 1))
        );

    }

}