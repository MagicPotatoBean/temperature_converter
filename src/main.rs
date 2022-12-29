use std::io;
fn main() {
    let mut input: f64;
    let mut result: f64;
    loop {
    println!("To convert from celcius to farenheit, type 1, else, type 2");
    let mode: i8 = read_i8();
        if mode == 1 {
            println!("Enter temperature in celcius");
            input = read_double();
            result = (input * 1.8) + 32.0;
            println!("{} degrees celcius is {} degrees farenheit", input, result);
        } else if mode == 2 {
            println!("Enter temperature in farenheit");
            input = read_double();
            result = (input - 32.0) / 1.8;
            println!("{} degrees farenheit is {} degrees celcius", input, result);
        } else {
            println!("Please only enter either a 1 or a 2.");
            continue;
        }
    }
}
fn read_double() -> f64 {
    let mut value: String = Default::default();
    io::stdin().read_line(&mut value).expect("Error reading line");
    let value: f64 = value.trim().parse().expect("Not a number");
    return value
}
fn read_i8() -> i8 {
    let mut value: String = Default::default();
    io::stdin().read_line(&mut value).expect("Error reading line");
    let value: i8 = value.trim().parse().expect("Not a number");
    return value
}