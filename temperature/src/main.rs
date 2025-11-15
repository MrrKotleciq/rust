use std::io;

fn celcius() {
    println!("Please input your temperature in celcius.");

    let mut c = String::new();
    io::stdin().read_line(&mut c).expect("Faild to read line");
    let c: u32 = c.trim().parse().expect("Please type a number!");
    println!("Your input temperature is: {c} celcius");


    let f = c*9/5 + 32;

    println!("Your temperature in Fahrenheit is: {f} degree.")
}

fn fahrenheit() {
    println!("Please input your temperature in fahrenheit.");

    let mut f = String::new();
    io::stdin().read_line(&mut f).expect("Faild to read line");
    let f: u32 = f.trim().parse().expect("Please type a number!");
    println!("Your input temperature is: {f} fahrenheit");


    let c = (f-32)*5/9;

    println!("Your temperature in Fahrenheit is: {c} degree.")
}

fn main() {
    'main: loop{ 
        println!("Please select an option: celcius [c] fahrenheit [f] exit [e]");
        
        let mut select = String::new();
        io::stdin().read_line(&mut select).expect("Faild t oread line");
        let select: char = select.trim().parse().expect("Please type a string"); 

        if select == 'c' {
            celcius();
        }
        else if select == 'f' {
            fahrenheit();
        }
        else if select == 'e' {
            break 'main;
        }
        else {
            println!("Please select proper option!")
        }
    }
}
