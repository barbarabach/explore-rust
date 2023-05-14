fn main() {
    println!("Hello, world!");
    let mut number = 9;
    while number != 0 {
        if number % 2 == 0 {
            number = number / 3;
        } else {
            number = number * 6;
        }
        print!("--> {}", number);
    }
    println!();
    println!("The end!");
}

