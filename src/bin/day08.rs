use std::env;
use std::borrow::Borrow;

const Y_SIZE :i32 = 6;
const X_SIZE :i32 = 25;
const LAYER_SIZE :i32 = X_SIZE * Y_SIZE;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Reading from file: {}", filename);
    let s = std::fs::read_to_string(filename).unwrap();
    let layer_size = Y_SIZE * X_SIZE;
    let layer_count = s.len() as i32/ layer_size;
    let v = s.trim();

    let mut layers = vec![];
    let mut res = vec![];

    for layer in 0..layer_count {
        let index = (layer * layer_size) as usize;
        let end_range = index + layer_size as usize;
        let layer : &str= &v[index..end_range];
        let zeros = get_digits(&layer, '0');
        let ones = get_digits(&layer, '1');
        let twos = get_digits(&layer, '2');
        let r = ones * twos;
        let size = zeros + ones + twos;
        res.push((zeros, ones, twos, r, size));
        layers.push(layer);
        print_layer(layer);
        println!("")
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

    let som =  merge_layers(layers);
    println!("S: {}", som);
    print_layer(&som);

}

fn merge_layers(layers :Vec<&str>) -> String {
    let mut result = String::new();
    for pos in 0..LAYER_SIZE {
        let mut pixel = '0';
        for layer in layers.iter() {
            let c_val = layer.chars().nth(pos as usize).unwrap();
            if c_val != '2' {
                pixel = c_val;
                break;
            }
        }
        result.push(pixel);
    }
    return result;
}

fn print_layer(layer :&str) {
    for y in 0..Y_SIZE {
        for x in 0..X_SIZE {
            print!("{}", layer.chars().nth((y * X_SIZE + x) as usize).unwrap())
        }
        println!("");
    }
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