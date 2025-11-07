use std::io;

// fn main() {
//     let x = 5;

//     let x = x + 1;

//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }

//     println!("The value of x is: {x}");
// }

fn main() {
    // let x = 2.0; //f64
    // let y: f32 = 3.0; //f32

    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    
    //division
    let _quotient = 56.7 / 32.2;
    let _truncated = -7 / 3;

    let _remainder = 43 % 5;
    // println!("truncated = {quotient}");

    let _t = true; //boolean
    let _f: bool = false;

    let tup: (i32, f64, u8) = (500, 6.4, 1); //tuple
    let (_x, y, _z) = tup;
    println!("\nThe value of y is: {y}\n");

    let tup2 = tup.2;
    println!("\nThe 3rd value of tup is: {tup2}\n");

    let a: [i32; 5] = [1, 2, 3, 4, 5]; //array
    let _b = [3; 5]; //=[3, 3, 3, 3, 3]
    let fourth = a[3];
    println!("This is the 4th element of 'a' array: {fourth}\n");


    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Faild to read line");
    let index: usize = input.trim().parse().expect("index entered was not a number");
    let element = a[index];

    println!("The value of the element at index {input} is: {element}");
}