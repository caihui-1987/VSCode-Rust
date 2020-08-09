//假定生男孩生女孩的比例是1:1，不能在孩子出生前选择孩子的性别，每队夫妻都希望至少有一个男孩，
//于是大家都采取这样一种生育策略，如果头胎是男孩，就不再生，如果头胎是女孩就生二胎，
//二胎是男孩就不生，是女孩就继续生，直到生出男孩为止，
//那么现在问，在这样一种策略下，整个社会是男孩多还是女孩多？
use rand::Rng;
fn main() {
    let xiaohai = shengxiaohai(100);
    println!("boy = {},girl = {}", xiaohai.boy, xiaohai.girl);
}
//定义一个结构体
struct Nannv {
    boy: u32,
    girl: u32,
}
//用n记录一共有多少对夫妻
fn shengxiaohai(n: u32) -> Nannv {
    let mut boy = 0; //记录一共生了几个男孩
    let mut girl = 0; //记录一共生了几个女孩

    for i in 0..n {
        let mut xiaohai: Vec<String> = Vec::new(); //定义一个vecter存放每对夫妻生的小孩的顺序
        loop {
            let j = rand::thread_rng().gen_range(1, 3); //用j记录生男孩是女孩，生男孩是2，生女孩是1
            match j {
                //当生的是女孩时继续循环
                1 => {
                    girl += 1;
                    xiaohai.push(String::from("girl"))
                }
                //当生的是男孩时跳出循环
                2 => {
                    boy += 1;
                    xiaohai.push(String::from("boy"));
                    println! {"第{}对夫妻生的小孩：\n{:?}\n",i+1,xiaohai};
                    break; //
                }
                _=>(),
            }
        }
    }
    Nannv { boy, girl }
}
