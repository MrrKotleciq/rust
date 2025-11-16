fn main() {
    
    
    // let mut n = "Hello";
    
    
    
    // let mut s = String::from(n);
   
   
   
    // n = "Hello, world!";
    // s.push_str(", world!");
    // println!("{n}");
    // println!("{s}");

    // let s = String::from("Hello");
    // let mut s2 = s.clone();
    
    // s2 = String::from("Ahoy");


    // println!("{s2}, world!");

let n = String::from("Hello"); //string move is valid even if made in diferent scope.

for _i in 1..2{
    let n = String::from("Albert's");
    let mut n1 = n.clone();
    n1.push_str(" world");
    println!("{n1}");
}

println!("{n}");

}




