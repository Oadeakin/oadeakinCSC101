// Rust program to determine annual incentive 

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("\nEnter your age: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let age:f32 =  input1.trim().parse().expect("Not a valid number");

    println!("\nEnter number years of experience: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let experience:f32 =  input2.trim().parse().expect("Not a valid number");

    if age >= 40.0 && experience >=10.0{
        println!("The incentive of the employee is N1_560_000");
    }
    else if age >= 40.0 && experience < 10.0{
        println!("The incentive of the employee is N100_000");
    }
    else if age >= 30.0 && age < 40.0 && experience >=10.0{
        println!("The incentive of the employee is N1_480_000");
    }
    else if age >= 30.0 && age < 40.0 && experience < 10.0{
        println!("The incentive of the employee is N100_000");
    }
    else if age < 30.0 && experience >= 10.0{
        println!("The incentive of the employee is N1_300_000");
    }
    else if age < 30.0 && experience < 10.0{
        println!("The incentive of the employee is N100_000");
    }
}
