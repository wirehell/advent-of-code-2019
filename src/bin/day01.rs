use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Reading from file: {}", filename);
    let f = File::open(filename).expect("Oops! Something went wrong");
    let file = BufReader::new(&f);

    let mut sum1 :i64 = 0;
    let mut sum2 :i64 = 0;

    for line in file.lines() {
        let mass :i64 = line.unwrap().parse().unwrap();
        sum1 += calculate_fuel(mass);
        sum2 += calculate_fuel_rec(mass);
    }
    println!("Sum fuel: {}", sum1);
    println!("Sum fuel rec: {}", sum2);
}

fn calculate_fuel(mass: i64) -> i64 {
    return mass / 3 - 2;
}


fn calculate_fuel_rec(mass: i64) -> i64 {
    let fuel = calculate_fuel(mass);
    if fuel <= 0 {
        return 0
    } else {
        return fuel + calculate_fuel_rec(fuel);
    }
}

#[cfg(test)]
mod tests {
    use crate::{calculate_fuel, calculate_fuel_rec};

    #[test]
    fn testcases() {
//For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
        assert_eq!(calculate_fuel(12), 2);
//For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
        assert_eq!(calculate_fuel(14), 2);
//For a mass of 1969, the fuel required is 654.
        assert_eq!(calculate_fuel(1969), 654);
//For a mass of 100756, the fuel required is 33583.
        assert_eq!(calculate_fuel(100756), 33583);
    }

    #[test]
    fn testrec() {
        assert_eq!(calculate_fuel_rec(14), 2);
        assert_eq!(calculate_fuel_rec(1969), 966);
        assert_eq!(calculate_fuel_rec(100756), 50346);
    }
}