//定义枚举类型
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
//定义函数
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
fn main() {
   
    println!("{}",value_in_cents(Coin::Quarter));
}
