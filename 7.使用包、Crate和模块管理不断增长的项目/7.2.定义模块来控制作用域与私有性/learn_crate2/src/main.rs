use mylib::factory::produce_refrigerator;
use mylib::factory::produce_refrigerator::produce_re;//更进一层
//另外一种简便的用法
use mylib::factory::produce_refrigerator as A;
use mylib::factory::*;//导入所有模块
fn main() {
    mylib::factory::produce_refrigerator::produce_re();//绝对路径
    produce_refrigerator::produce_re();//使用use，推荐使用
    A::produce_re();//另外一种简便的用法
    produce_re();
    println!("Hello, world!");
}
