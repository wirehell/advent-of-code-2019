
type Code = Vec<i8>;


fn to_code(val :u32) -> Code {
    assert!(val <= 999999);
    assert!(val >= 100000);

    let mut code:Code = vec![];
    let mut rest = val;
    let mut div = 100000;
    for i in 0..6 {
        let digit = rest / div;
        rest = rest % div;
        div = div / 10;
        code.push(digit as i8);
    }
    return code;
}

fn is_valid(code :Code) -> bool {
    return has_adjacent(&code) && is_non_decreasing(&code)
}

fn has_adjacent(code :&Code) -> bool {
    for i in 0..5 {
        let d = code[i];
        if d == code[i+1]
            && (i == 0 || code[i-1] != d)
            && (i == 4 || code[i+2] != d) {
            return true;
        }
    }
    return false;
}

fn is_non_decreasing(code :&Code) -> bool {
    for i in 0..5 {
        if code[i] > code[i+1] {
            return false;
        }
    }
    return true;
}

fn main() {
    let mut count = 0;
    for c in 353096..=843212 {
        if is_valid(to_code(c)) {
            count += 1;
        }
    }

    println!("Count: {}", count);
}


#[cfg(test)]
mod tests {
    use crate::{to_code, is_valid};

    #[test]
    fn test_to_code() {
        assert_eq!(to_code(123456), vec![1,2,3,4,5,6]);
    }

    #[test]
    fn test_is_valid() {
        assert_eq!(is_valid(to_code(112233)), true);
        assert_eq!(is_valid(to_code(123444)), false);
        assert_eq!(is_valid(to_code(111122)), true);
    }

}