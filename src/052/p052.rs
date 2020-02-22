fn digits(number: u32) -> Vec<u32> {
    let temp = number.to_string();
    let mut digits = temp.chars()
                         .map(|d| d.to_digit(10).unwrap())
                         .collect::<Vec<u32>>();
    digits.sort();    
    digits
}

fn condition(x: u32) -> bool {
    let a = digits(x) == digits(2 * x);
    let b = digits(x * 3) == digits(x * 4);
    let c = digits(x * 5) == digits(x * 6);
    a && b && c
}

fn solve() -> i32 {
    let mut number = 1;
    while !condition(number) {
        number += 1;
    }
    number
}

fn main() {
    let result = solve();
    println!("{}", result);
}
