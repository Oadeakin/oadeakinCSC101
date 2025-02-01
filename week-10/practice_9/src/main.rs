//define dimensions of a rectangle
struct Rectangle{
    width:u32, height:u32
}

//logic to calucate area of a recatangle
impl Rectangle {
    fn area(&self)->u32{
        //use the . operator to fetch the value of a file dvia the self keyword
        self.width * self.height
    }
}
fn main() {
    //instanatiate the structre
    let small = Rectangle{
        width:10,
        height:20
    };
    //print the rectangle's area
    println!("width is {} \n height is {} \n area of Rectangle
    is {}",small.width,small.height,small.area());
}
