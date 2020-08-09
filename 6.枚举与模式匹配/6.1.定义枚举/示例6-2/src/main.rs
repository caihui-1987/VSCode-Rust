//一个Message枚举，其每个成员都存储了不同数量和类型的值
enum Message{
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}
impl Message{
    fn call (&self){
        println!("hehhe");
    }
}
fn main() {
    let a = Message::ChangeColor(1,3,4);
    a.call();
    println!("Hello, world!");
}
