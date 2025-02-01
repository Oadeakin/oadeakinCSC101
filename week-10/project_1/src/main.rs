struct Laptops{
    hp:i32,
    ibm:i32,
    toshiba:i32,
    dell:i32
}

impl Laptops{
    fn total_cost(&self)-> i32{
        3*self.hp + 3*self.ibm + 3*self.toshiba + 3*self.dell
    }
}
fn main() {
    let purchases = Laptops{
        hp:650_000,
        ibm:755_000,
        toshiba:555_000,
        dell:850_000
    };
    println!(" HP:{} \n ibm:{} \n toshia:{} \n dell:{} \n Total cost of purchase is {}",purchases.hp,purchases.ibm,purchases.toshiba,purchases.dell,purchases.total_cost());
}
