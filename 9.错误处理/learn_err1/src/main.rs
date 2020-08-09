//1.Rust语言将错误分为两个类别：可恢复的和不可恢复的错误
//（1)可恢复的错误通常代表向用户报告错误和重试操作是合理的情况，如未找到文件：rust中使用Result<T,E>来实现
//（2）不可恢复错误是bug的同义词，如尝试访问数组结尾的部分，Rust中通过panic！来实现
use std::fs::File;
fn main() {
//2.panic!
    // panic!("crash here");
//3.使用BACKTRACE=1
//4.Result<T,E>
//enum Result<T,E>{
//     Ok(T),
//     Err(E),
// }
// let f = File ::open("hello.txt");
// let r = match f {
    // Ok(file) => file,
    // Err(error) => panic!("err:{:?}",error),
// };
//5.简写
// let f = File::open("hello.txt").unwrap();//第一种方式
let g = File::open("hello.txt").expect("Failed to open hello.txt");
}
