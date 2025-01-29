fn main() {
    let office = vec!["Intern","Administrator","Senior Administrator","Office Manager
", "Director","CEO"];

let academic = vec!["null","Research assistant","PhD Candidate","Post-Doc Researcher
", "Senior Lecturer","Dean"];
 
let lawyer = vec!["Paralegal","Junior associate","Associate","Senior Associate 1-2
", "Senior Associate 3-4","Partner"];

let teacher = vec!["Placement","Classroom teacher","Senior teacher","Leading Teacher", "Deputy principal","Principal"];

let level = vec!["1-2","3-5","5-8","8-10","10-13","SES",];


 let mut input1 = String::new();

 println!("\nPlease state your Public servant class (Office Administrator, Academic, Lawyer or Teacher)");
 std::io::stdin().read_line(&mut input1).expect("Not a valid input");
 let class = input1.trim();
  
 if class == "Office Administrator"{
     println!("\n Intern , Administrator , Senior Administrator , Office Manager , Director , CEO");
  }

  if class == "Academic"{
     println!("\n N/A , Research assistant , PhD Candidate , Post-Doc Researcher , Senior Lecturer , Dean");
  }

  if class == "Lawyer"{
     println!("\n Paralegal , Junior associate , Associate , Senior Associate 1-2 , Senior Associate 3-4 , Partner");
  }

  if class == "Teacher"{
     println!("\n Placement , Classroom teacher , Senior teacher , Leading Teacher , Deputy principal , Principal");
  }


 
 let mut input2 = String::new();
 println!("\n What position above do you currently occupy in the {} class? ", class);
 std::io::stdin().read_line(&mut input2).expect("Not a valid input");
 let pos = input2.trim();
 
 let mut input3 = String::new();
 println!("How many years have you spent at {} class?", class);
 std::io::stdin().read_line(&mut input3).expect("Not a valid input");
 let exp:i64 = input3.trim().parse().expect("Not a valid input");


 if exp <3 {println!("You hold an APS Position 1-2");
}

if exp >2 && exp <=5 {println!("You hold an APS Position 3-5");
}

if exp >5 && exp <= 8 {println!("You hold an APS Position 5-8");
}

if exp >8 && exp <= 10 {println!("You hold an EL1 Position 8-10");
}

if exp >9 && exp <= 13 {println!("You hald an EL2 Position 10-13");
}
if exp >13 {println!("You hold an SES Position");
}

}
