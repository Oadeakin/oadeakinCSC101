fn main() {
    let p: f64 = 510_000.0;
    let r: f64 = 5.0;
    let n: i32 = 3; 

    // Depreciation
    let a = p * (1.0 - (r / 100.0)).powi(n);
    println!("Amount is {}", a);
    let d = a - p;
    println!("New value of car is {}", d);
}
