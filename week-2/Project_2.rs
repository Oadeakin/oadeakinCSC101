fn main(){
	let t:f64 = 450_000.0;
	let m:f64 = 1_500_000.0;
	let h:f64 = 750_000.0;
	let d:f64 = 2_850_000.0;
	let a:f64 = 250_000.0;

	//sum
	let s = (t * 2.0) + (m * 1.0) + (h * 3.0) + (d * 3.0) + (a * 1.0);
	println!("Sum of sales record is {}", s);

	//average
	let av = s / 10.0;
	println!("Average of the sales record is {}", av);
}