// fn main() {
    // let number = 8;

    // if number % 4 == 0 {
    // println!("number is divisible by 4");
    // }    
    // else if number % 3 == 0 {
    //     println!("number is divisible by 3");
    // }
    // else if number % 2 == 0 {
    //     println!("number is divisible by 2");
    // }
    // else {
    //     println!("number is not divisible by 4, 3 or 2")
    // }

    // let condition = true;
    // let number = if condition { "asd" } else { "klakla" };

    // println!("The value of number is: {number}")
    
//     let mut counter = 0;
    
//     let result = loop {
//         counter += 1;
        
//         if counter == 10{
//             break counter * 2;
//         }
//     };
        
//     println!("The result is {result}")
    
// }


// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {count}");
// }

// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{number}!");

//         number -= 1;
//     }

//     println!("LIFTOFF!!!");
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];
    
//     // the code could cause bug and its slower then the new one
    
//     //let mut index = 0;
//     // while index < 5 {
//     //     println!("The value is: {}", a[index]);

//     //     index += 1;
//     // }

//     for element in a {
//         println!("The value is: {}", element)
//     }
//     //0.03s time saved
// }

fn main() {
    for number in (1..4).rev() {
        println!("{number}!")
    }
    println!("LIFTOFF!!!")
}
