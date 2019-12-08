use std::env;

const Y_SIZE :i32 = 6;
const X_SIZE :i32 = 25;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Reading from file: {}", filename);
    let s = std::fs::read_to_string(filename).unwrap();
    let layer_size = Y_SIZE * X_SIZE;
    let layers = s.len() as i32/ layer_size;
    let v = s.trim();

    let mut res = vec![];
    for layer in 0..layers {
        let index = (layer * layer_size) as usize;
        let end_range = index + layer_size as usize;
        let layer : &str= &v[index..end_range];
        let zeros = get_digits(&layer, '0');
        let ones = get_digits(&layer, '1');
        let twos = get_digits(&layer, '2');
        let r = ones * twos;
        let size = zeros + ones + twos;
        res.push((zeros, ones, twos, r, size));
    }
    let mut min = 999999;
    let mut max_value = 0;
    for val in res {
        if val.0 < min {
            println!("New min: {:?}", val);
            min = val.0;
            max_value = val.3;
        }
    }
    println!("Res: {:?}", max_value);
}


fn get_digits(s :&str, d: char) -> i32 {
    let mut count = 0;
    for c in s.chars() {
       if c == d {
           count += 1;
       }
    }
    return count;

}

#[cfg(test)]
mod tests {

}