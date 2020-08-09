fn main() {
    //1.创建空的vector：Vector<T>
    let v: Vec<i32> = Vec::new();

    //2.创建包含初始值的vector
    let v = vec![1, 2, 3];
    println!("{:?}", v);

    //3.丢弃vector
    {
        let v1 = vec![1, 3, 4, 5];
    }

    //4.读取元素
    let one = v[0];
    println!("one = {}", one);
    println!("{:?}", v);
    // println!("one = {}", *one);
    //推荐的方法match
    match v.get(1) {
        Some(value) => println!("value = {}", value),
        _ => println!("None"),
    }

    //5.更新
    let mut v2: Vec<i32> = Vec::new();
    v2.push(1);
    v2.push(2);
    println!("{:?}", v2);
    v2[1] = 23;
    println!("更新后{:?}", v2);

    //6.遍历
    //不可变的遍历
    for i in &v2 {
        println!("i = {}", i);
    }
    //可变的遍历
    for i in &mut v2 {
        *i += 1;
        println!("i = {}", i);
    }

    //7.使用枚举
    enum Context {
        Text(String),
        Float(f32),
        Int(i32),
    }
    let a = vec![
        Context::Text(String::from("shdhe")),
        Context::Int(32),
        Context::Float(0.23),
         ];

    //8.补充(报错)
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);
    println!("first = {}", first);
}
