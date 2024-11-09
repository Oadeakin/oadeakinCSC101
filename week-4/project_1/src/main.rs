// Nature of roots of a quadratic equation
use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter value of a: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter value of b: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter value of c: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    let mut d:f32 = b.powf(2.0) - (4.0 * a * c);
    d = d.sqrt();
    println!("Value of discriminant: {}", d);

    if d > 0.0{
        println!("There are two distinct roots");
    }
    else if d == 0.0{
        println!("There is exactly one root");
    }
    else if d < 0.0{
        println!("There is no real root");
    }

}
