use std::io

fn main(){
    let mut name = string::new();
    println!("Enter your name: ");
    io::stdin().read_line(&mut name).expect("Not a valid string");

    let mut date_of_birth = string::new();
    println!("Enter your date of birth: ");
    io::stdin().read_line(&mut date_of_birth).expect("Not a valid string");
    let age:f32 =  date_of_birth.trim().parse().expect("Not a valid number");

    let mut email_address = string::new();
    println!("Enter your email adress: ");
    io::stdin().read_line(&mut email_address).expect("Not a valid string");

    let mut phone_number= string::new();
    println!("Enter your phone number: ");
    io::stdin().read_line(&mut phone_number).expect("Not a valid string");
    let number:i32 =  phone_number.trim().parse().expect("Not a valid number");

    let mut siblings= string::new();
    println!("Enter your number of siblings: ");
    io::stdin().read_line(&mut siblings).expect("Not a valid string");
    let siblings:i32 =  siblings.trim().parse().expect("Not a valid number");

    let mut children= string::new();
    println!("Enter your number of children: ");
    io::stdin().read_line(&mut children).expect("Not a valid string");
    let children:i32 =  children.trim().parse().expect("Not a valid number");

    let mut medical= string::new();
    println!("Enter your medical diagnosis: ");
    io::stdin().read_line(&mut medical).expect("Not a valid string");

    let mut residence= string::new();
    println!("Enter your village of residence: ");
    io::stdin().read_line(&mut residence).expect("Not a valid string");
}