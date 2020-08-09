#[derive(Debug)]
struct Dog {
    name:String,
    weight:f32,
    height:f32,
}
impl Dog{
    fn get_name(&self)->&str{
        &(self.name[..4])
    }
    fn get_weight(&self)->f32{
        self.weight
    }
    fn get_height(&self)->f32{
        self.height
    }
    fn show(){
        println!("oh oh oh !")
    }
}
fn main() {
    let dog = Dog{
        name:String::from("wangcai"),
        weight:100.0,
        height:70.5,
    };
    println!("打印结构体示例为：\n{:#?}",dog);
    println!("{}",dog.get_height());//使用结构体方法
     println!("{}",dog.get_name());//使用结构体方法
     Dog::show();
}
