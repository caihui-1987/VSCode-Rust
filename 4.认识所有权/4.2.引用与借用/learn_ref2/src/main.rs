//在任意给定时间，在有了可变引用之后不能再有不可变引用
//不能出现悬垂引用
fn main() {
    let ref_s= dangle();

    println!("Hello, world!");
}
fn dangle()->&String{
    let s = String::from("hello");
&s
}