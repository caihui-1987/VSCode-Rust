fn main() {
    let rect1 = (30,50);

    println!("The area of the rectangle is {} square pixels.",area(rect1));
}
//使用元组来定义长方形的宽高
fn area(dimensions:(u32,u32))->u32{
    dimensions.0*dimensions.1
}