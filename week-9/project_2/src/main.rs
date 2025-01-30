use std::fs::File;
use std::io::{Write, Result};

fn main() -> Result<()> {
    let name = vec!["Oluchi Mordi", "Dams Aliyu", "Shania Bolade", "Adelunke Gold","Blanca Edemoh",];
    let matric = vec![ "ACC10211111","EOC10110101","CSC10328828","EEE11020202","MEE10202001"];
    let dept = vec!["Accounting", "Economics", "Computer","Electrical", "Mechanical"];
    let level = vec![300, 100, 200, 200, 100];

    let mut file = File::create("students.txt")?;

    let header = "List of Students\n\n";
    file.write_all(header.as_bytes())?;
    println!("{}", header);

    for i in 0..name.len() {
        let student_data = format!(
            "Name: {}\nMatric: {}\nDepartment: {}\nLevel: {}\n\n",
            name[i],
            matric[i],
            dept.get(i).unwrap_or(&"Unknown"),
            level[i]
        );

        file.write_all(student_data.as_bytes())?;
        println!("{}", student_data);
    }

    println!("Data successfully saved to 'students.txt'");
    Ok(())
}