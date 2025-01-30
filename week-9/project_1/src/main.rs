use std::fs::File;
use std::io::Write;

fn main() {
    let larger = vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let stout = vec!["Legend", "Turbo", "Williams"];
    let non_alcoholic = vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"];

    let mut file = File::create("breweries.txt").expect("Failed to create file");

    file.write_all("List of highest quality drinks\n\n".as_bytes())
        .expect("Failed to write to file");

    file.write_all("Larger:\n".as_bytes()).expect("Failed to write to file");
    for drink in larger {
        file.write_all(format!("- {}\n", drink).as_bytes())
            .expect("Failed to write to file");
    }

    file.write_all("\nStout:\n".as_bytes()).expect("Failed to write to file");
    for drink in stout {
        file.write_all(format!("- {}\n", drink).as_bytes())
            .expect("Failed to write to file");
    }

    file.write_all("\nNon-Alcoholic:\n".as_bytes())
        .expect("Failed to write to file");
    for drink in non_alcoholic {
        file.write_all(format!("- {}\n", drink).as_bytes())
            .expect("Failed to write to file");
    }

    println!("Data written to file successfully.");
}