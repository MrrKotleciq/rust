use std::io;

fn main() {
    println!("Please input your temperature in celcius.");

    let mut c = String::new();
    io::stdin().read_line(&mut c).expect("Faild to read line");
    let c: u32 = c.trim().parse().expect("Please type a number!");
    println!("Your input temperature is: {c} celcius");


    let f = c*9/5 + 32;

    println!("Your temperature in Fahrenheit is: {f} degree.")
}
