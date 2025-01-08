use std::io;

fn main(){
    //infromation
    println!("Below is the list of medical diagnosis and charges for each residence");
    println!("Alzeheimer 1,200,00 Akpabom 20%");
    println!("Arrhythmia 550,000 Ngbauji 5%");
    println!("Chronic Kidney Disease 1,500,000 Atabrikang 15%");
    println!("Diabetes 800,000 Okorobilom 10%");
    println!("Arthritis 450,000 Emeremen 10%");

    let mut input1 = String::new();
    println!("Enter your name: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let _name = input1.trim();

    println!("Enter your birth year:"); 
    let mut input2 = String::new(); 
    io::stdin().read_line(&mut input2).expect("Failed to read line"); 
    let birth_year: u32 = input2.trim().parse().expect("Invalid number");
    let current_year = 2024;

    let _age = current_year - birth_year;
    println!("Your age is: {}", _age);
    
    let mut input4 = String::new();
    println!("Enter your email adress: ");
    io::stdin().read_line(&mut input4).expect("Not a valid string");
    let _email = input4.trim().to_lowercase();

    let mut input5 = String::new();
    println!("Enter your phone number: ");
    io::stdin().read_line(&mut input5).expect("Not a valid string");
    let _number: u32 = input5.trim().parse().expect("Invalid number");
    
    let mut input6 = String::new();
    println!("Enter your number of siblings: ");
    io::stdin().read_line(&mut input6).expect("Not a valid string");
    let _siblings: u32 = input6.trim().parse().expect("Invalid number");
    
    let mut input7 = String::new();
    println!("Enter your number of children: ");
    io::stdin().read_line(&mut input7).expect("Not a valid string"); 
    let _children: u32 = input2.trim().parse().expect("Invalid number");   

    let mut input8 = String::new();
    println!("Enter your medical diagnosis: ");
    io::stdin().read_line(&mut input8).expect("Not a valid string");
    let _med = input8.trim().to_lowercase();
    
    let mut input9 = String::new();
    println!("Enter your village of residence: ");
    io::stdin().read_line(&mut input9).expect("Not a valid string");
    let _v = input9.trim().to_lowercase();
    
    //calculation
    let al = 1_200_000.0;
    let ar = 550_000.0;
    let ckd = 1_500_000.0;
    let d = 800_000.0;
    let art = 450_000.0; 

    
    if _med.as_str() == "alzeheimer" && _age > 50 && _v.as_str() == "akpabom" && _children > 4{
    let _rate = 0.8;
    let _calc = al * _rate;
    println!("You receive a 20% discount with an amount of: {}", _calc);
   }
   else if _med.as_str() == "arrythmia" && _age == 50 && _v.as_str() == "nbgauji" && _siblings > 4{
    let _rate = 0.95;
    let _calc = ar * _rate;
    println!("You receive a discount of 5% with an amount of: {}", _calc);
   }

   else if _med.as_str() == "ckd" && _age > 40 && _v.as_str() == "atabrikang" && _children > 3 && _siblings > 3{
    let _rate = 0.85;
    let _calc = ckd * _rate;
    println!("You receive a discount of 15% with an amount of: {}", _calc);
   }

   else if _med.as_str() == "diabetes" && _age > 28 && _age < 45 && _v.as_str() == "okorobilom" && _children >= 2 &&  _children <= 4{
    let _rate = 0.9;
    let _calc = _rate * d;
    println!("You receive a discount of 10% with an amount of: {}", _calc);
   }
    
   else if _med.as_str() == "arthritis" && _age > 58 && _v.as_str() == "emeremen" && _siblings > 5 && _children > 5 {
    let _rate = 0.9;
    let _calc = art * _rate;
    println!("You receive a discount of 10% with an amount of: {}", _calc)
   }
   else if _med.as_str() == "alzeheimer"{
    println!("Your amount is 1,200,000");
}
else if _med.as_str() == "arrythmia"{
    println!("Your amount is 550,00");
}
else if _med.as_str() == "ckd"{
    println!("Your amount is 1,500,000");
}
else if _med.as_str() == "diabetes"{
    println!("Your amount is 800,000");
}
else if _med.as_str() == "arthritis"{
    println!("Your amount is 450,000");
}

}