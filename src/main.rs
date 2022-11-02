use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let a = &args[1];
    let b = &args[2];
    let c = &args[3];

    three_strings(a, b, c);
    // dbg!(args);
}

fn three_strings(a: &str, b: &str, c: &str) {
    println!("value 1: {}", a);
    println!("value 2: {}", b);
    println!("value 3: {}", c);
}
