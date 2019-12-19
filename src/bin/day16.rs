use std::env;
use std::str::FromStr;
use itertools::{repeat_n, Itertools};
use std::char;


fn main() {
    let filename = "data/day16/input.txt";
    println!("Reading from file: {}", filename);
    let s = std::fs::read_to_string(filename).unwrap();

    let repeated = s.trim().repeat(10000);
    let count = i32::from_str(&repeated[0..7]).unwrap();
    println!("Count is :{}", count);
    let sl :Vec<i32> = split_and_parse(&repeated).iter().dropping(count as usize)
        .cloned()
        .collect();

    let result = solve_last(&sl);
    print_matrix();
    println!("Result: {:?}", &result[0..8]);
}

fn solve_last(v :&Vec<i32>) -> String {
    let mut wv = v.clone();
    let size = wv.len();
    for j in 0..100 {
        for i in 1..wv.len() {
            wv[size - 1 - i] = i32::abs((wv[size - 1 - i] + wv[size - i]) % 10);
        }
    }
    return wv.iter().map(|x| {
        char::from_digit(*x as u32, 10).unwrap()
    }).join("");
}

fn solve(v :&Vec<i32>) -> String {
    let res = fft_n(v, 100);
    let chopped :Vec<String> = res.iter()//.take(8)
        .map(|x| i32::to_string(x))
        .collect();
    let result : String = chopped.join("");
    return result;
}

fn fft_n(v :&Vec<i32>, phases: i32) -> Vec<i32> {
    let mut res = v.clone();
    for p in 0..phases {
//        println!("Result: {:?}", &res);
        res = fft(&res)
    }
    return res;
}

fn print_matrix() {
    let cyc = vec![0, 1, 0, -1];
    let base = cyc.iter().cloned().cycle();

    let len = 100;
    for i in 1..=len {
        let base = cyc.iter().cloned().cycle();
        let p = base.map(|x| repeat_n(x, i as usize)).flatten().dropping(1);
        let b :Vec<i32> = p.take(len).collect();
        println!("{:?}", b);
    }
}

fn fft(v :&Vec<i32>) -> Vec<i32> {

    let mut res = vec![];
    let cyc = vec![0, 1, 0, -1];

    for i in 1..=v.len() {
        let base = cyc.iter().cloned().cycle();
        let p = base.map(|x| repeat_n(x, i as usize)).flatten().dropping(1);
        let sum :i32 = v.iter().zip(p).map(|v|  {
 //           print!("{:?}", &v);
            let (x,y) = v;
            (x * y)
        }).sum();
        res.push(i32::abs(sum % 10));
    }

    return res;
}

fn split_and_parse(s :&str) -> Vec<i32> {

   let split = s.trim().chars();
   return split.map(|x| x.to_digit(10).unwrap() as i32).collect();
}


#[cfg(test)]
mod tests {
    use crate::{split_and_parse, fft, fft_n, solve, solve_last};

    #[test]
    fn test() {
        assert_eq!(split_and_parse(&"123"), [1,2,3])
    }

    #[test]
    fn test_last() {
        let v = split_and_parse(&"5518");
        let res = solve_last(&v);
        assert_eq!(res, String::from("9498"));
    }
    #[test]
    fn test_last2() {
        let v = split_and_parse(&"111111111111111111111111111111111111111111");
        let res = solve_last(&v);
        assert_eq!(res, String::from(     "000000000000005551666611116666111166661111"));
    }

    #[test]
    fn test_2() {
        let v = split_and_parse(&"12345678");
        let res = fft_n(&v, 2);
        assert_eq!(res, vec![3, 4, 0, 4, 0, 4, 3, 8]);
    }


    #[test]
    fn test_l1() {
        let v = split_and_parse(&"80871224585914546619083218645595");
        assert_eq!(solve(&v).as_str(), "24176176");
    }

    #[test]
    fn test_l2() {
        let v = split_and_parse(&"11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111");
//
 //                                             "73745418557257259149466599639917"
//                                               0000000000xx00x0xxx000000000xxxx

        assert_eq!(solve(&v).as_str(), "73745418");
    }

}