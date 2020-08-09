//使用match表达式处理可能返回的Result成员 
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file)=>file,
        Err(error)=> {
            panic!("Problem opening the file:{:?}",error)
        },
    };
}
