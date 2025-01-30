use std::fs::File;
use std::io::{Write, Result};

fn main() -> Result<()> {
    let name = vec!["Aigbogun Alamba Daudu", "Murtala Afeez Benud", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazawu Faith Etieye"];
    let ministry = vec![ "Internal Affairs","Justice","Defense","Power & Steel","Petroleum"];
    let geo = vec!["South West", "North East", "South South","South West", "South East"];

    let mut file = File::create("efcc.txt")?;

    let header = "List of commissioners with ministries and geopolitical zones\n\n";
    file.write_all(header.as_bytes())?;
    println!("{}", header);

    for i in 0..name.len() {
        let minister_data = format!(
            "Name: {}\nMinistry: {}\nGeo Political Zone: {}\n\n",
            name[i],
            ministry[i],
            geo.get(i).unwrap_or(&"Unknown"),
        );

        file.write_all(minister_data.as_bytes())?;
        println!("{}", minister_data);
    }

    println!("Data successfully saved to 'minister.txt'");
    Ok(())
}