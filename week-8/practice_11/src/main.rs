fn main() {
    //an array of numbers
    let numbers = [1,2,3,4,5];
    println!("Original arrage = {:?}",numbers);

    //create a slice of 2nd and 3rd element
    let slice1 = &numbers[1..3];
    println!("2nd and 3rd elements slice = {:?}",slice1);

    //omit the start index
    let slice2 = &numbers[..3];
    //This means the slice starts from the 0 index and goes up to index 3 (exclusive)
    println!("index 0 to 3 slice = {:?}",slice2);

    //omit the end index
    let slice3 = &numbers[..3];
    //This means the slice starts from index 2 and goes up to index 5 (exclusive)
    println!("index 2 to index 5 sliced = {:?}",slice3);

    //omit the start index and the end index
    //reference the whole array
    let slice4 = &numbers[..];
    //This means the slice stars from index 0 and goes up to index 5 (exlclusive)
    println!("index 0 to index 5 slice = {:?}",slice4);
}
