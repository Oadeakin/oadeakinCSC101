fn main() {
    let mut a:i32 = 25;
    let mut b:i32 = 15;

    if a > 0{
        a+=3;
        b-=2;
        let c = a + b;
        a = c / 2;
        b = a * 3;
        println!("a, c, b  are {} {} {}",a,c,b);
    }
}
