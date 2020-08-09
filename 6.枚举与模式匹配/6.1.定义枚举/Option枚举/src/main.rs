fn main() {
    
    let y:Option<i32> = Some(5);
    let y = plus_one(y);
}
fn plus_one(x:Option<i32>)->Option<i32>{
    match x{
        None=> None,
        Some(i)=>Some(i+1),
    }
}