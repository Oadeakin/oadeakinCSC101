fn main() {
    println!("\n Welcome to the Ernst & Young (EY) Global Limited. Fill out the job application sincerely. Application is only open for the first 50 people.");

    let mut highest_experience = 0;
    let mut most_experienced_person = String::new();

    for i in 1..=50{
    println!("\n Applicant #{}",i);
    
    let mut input1 = String::new();
    println!("What is your name?");
    std::io::stdin().read_line(&mut input1).expect("Not a valid String");
    let name = input1.trim().to_string();

    let mut input2 = String::new();
    println!("How old are you?");
    std::io::stdin().read_line(&mut input2).expect("Not a valid input");
    let age:i64 = input2.trim().parse().expect("Not a valid input");

    let mut input3 = String::new();
    println!("How many years have you been working?");
    std::io::stdin().read_line(&mut input3).expect("Not a valid input");
    let exp:i64 = input3.trim().parse().expect("Not a valid input");

    if exp > highest_experience {
        highest_experience = exp;
        most_experienced_person = name.clone();
    }

    println!("\nThank you,{}. Here is the summary of your application: \nAge: {} \n Experience: {} years\n", name,age,exp);
    }

    println!("\nThe person with the highest number of experiences is {} with {} years of experience.",most_experienced_person, highest_experience);

    println!("\nThe application is closed now.");
}