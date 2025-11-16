use std::io;

fn fibonacci(n: u32) -> u32 {
    if n == 0{
        0
    }
    else if n == 1 || n == 2{
        1
    }
    else{
        fibonacci(n-1) + fibonacci(n-2)
    }
}


fn main() {
    println!("Input n'th fibonacci");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Faild to read line");
    let n: u32 = n.trim().parse().expect("Please type a positive number");

    let out = fibonacci(n);
    println!("{out}")

}
