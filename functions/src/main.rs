// fn another_function() {
//     println!("Another function")
// }

// fn another_function(x: i32, unit_label: char) {
//     println!("The value of x is: {x}{unit_label}")
// }
// fn main() {
//     another_function(5, 'h');
// }

fn main() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is {y}");
}