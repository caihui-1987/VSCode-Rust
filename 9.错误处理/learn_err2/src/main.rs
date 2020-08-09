//1.当编写一个函数，但是该函数可能会失败，此时除了在函数中处理错误外，还可以将错误传给调用者让调用者决定如何处理，这被称为传播错误。
use std::io;
use std::io::Read;
use std::fs::File;
fn main() {
    println!("Hello, world!");
    let r = read_username_from_file2();
    match r {
        Ok(s)=>println!("s = {}",s),
        Err(e)=>println!("err = {:?}",e),
    }
}
fn read_username_from_file() -> Result<String,io::Error>{
    //打开文件
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file)=>file,
        Err(error)=> return Err(error),
    };
    //读取文件
    let mut s = String::new();
    match f.read_to_string(&mut s){
        Ok(_)=> Ok(s),
        Err(error) => Err(error),
    }
}
//2.传播错误的简写方式，提倡的方式
fn read_username_from_file2() -> Result<String,io::Error>{
    //打开文件
    let mut f = File::open("hello.txt")?;
    //读取文件
    let mut s = String::new();
     f.read_to_string(&mut s)?;
     Ok(s)
}
//3.更进一步的简写
fn read_username_from_file3() -> Result<String,io::Error>{
   
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
     Ok(s)
}
//4.什么时候用panic！，什么时候用Result
//5.Option和Result