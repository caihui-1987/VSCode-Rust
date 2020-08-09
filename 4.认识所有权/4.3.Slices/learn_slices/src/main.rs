
fn main() {
    //1.字符串slice是String中一部分值的引用
    let s = String::from("hello word");
    //取“hello”
    let h = &s[0..5];
    println!("h = {}", h);
    //取“word”
    let w = &s[6..];
    println!("w = {}", w);
    let ss = String::from("你好");
    let w1 = &ss[..3]; //一个汉字三个字节
    println!("w1 = {}", w1);

    //2.字面值
    let s3 = "hh"; //&str

    //3.其他类型的slices
    let a = [1, 2, 3, 4];
    let sss = &a[1..3];
    println!("sss = {:?}", sss);
}
