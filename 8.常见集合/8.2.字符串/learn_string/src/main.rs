fn main() {
    //1.创建一个空String
    let mut s0 = String::new();
    s0.push_str("hello");
    println!("s0 = {}", s0);

    //2.通过字面值创建一个String
    //2.1使用String：；from（）
    let s1 = String::from("init some thing");
    println!("{}", s1);
    //2.2使用to_string的方式
    let s1 = "init some thing".to_string();
    println!("{}", s1);

    //3.更新String
    //3.1push_str
    let mut s2 = String::from("hello");
    s2.push_str(",world");
    let ss = " !".to_string();
    s2.push_str(&ss);
    println!("{}", s2);
    //3.2push
    let mut s2 = String::from("tea");
    s2.push('m'); //注意单引号
                  // s2.push('mx');//error
                  // s2.push("m");//error
    println!("{}", s2);
    //3.3使用“+”合并字符串
    let s1 = "hello".to_string();
    let s2 = String::from(", world");
    let s3 = s1 + &s2; //注意s2是引用
    println!("s3 = {}", s3);
    //3.4format宏的使用
    let s341 = String::from("tic");
    let s342 = String::from("tac");
    let s343 = String::from("toe");
    let s344 = format!("{}-{}-{}", s341, s342, s343);
    println!("s344 = {}", s344);

    //4String索引
    let s4 = String::from("hello");
    //let s41 = s4[0];//error
    let s4 = String::from("你好");
    //let s41 = s4[0];//error

    //5.str索引
    let hello = String::from("你好");
    let h5 = &hello[..3];
    println!("{}", h5);

    //6.遍历字符串
    //6.1chars
    let s4 = String::from("你好");
    for c in s4.chars() {
        println!("c = {}", c);
    }
    //6.2bytes
    let s5 = String::from("hello, world");
    for b in s5.bytes() {
        println!("b = {}", b);
    }
}
