use rand::Rng;
use std::io;

fn main() {
    let mut a = String::new();
    let mut b = vec!["张三", "李四", "王五", "朱六", "卓七", "赵八"];
    let mut b1:Vec<String> = Vec::new();
    for i in &b{
        b1.push(i.to_string());
    }
    let c = 7;
    let n = true;
     io::stdin()
        .read_line(&mut a)
        .expect("输入错误");
        
    for i in 0..5{
       
       let y=choujiang(&a, b1, c, n);
       println!("{}是{:?}", y.0, y.2);
        b1 = y.1;

    }

    // let x = choujiang(&a, b, c, n);
    // println!("{}是{:?}", x.0, x.2);
}
//a为奖项名称，b为参与抽奖人员名单，c为该奖项一次抽几人，n记录中奖的人是否还能再抽奖
fn choujiang<'a>(a: &String, b: Vec<String>, c: u8, n: bool) -> (String, Vec<String>, Vec<String>) {
    let mut b_len = b.len(); //记录参与抽奖的人数
    let mut d: Vec<String> = Vec::new(); //存放中奖人员名单
    let mut b1:Vec<String> = Vec::new();//存放抽奖后还能参与抽奖的人员
    for i in &b{
        b1.push(i.to_string());
    }
    for i in 0..c {
        let rng1 = rand::thread_rng().gen_range(0, b_len); //取参加抽 奖人员的随机值
        d.push(b1[rng1]); //将抽中的人员放入中奖名单
        if !n {
            //判断中奖的人是否还能再抽奖
            b.remove(rng1); //移除其再次抽奖资格
            b_len -= 1;
        }

    }
    return (*a, b1, d); //a是中奖名称，b本轮抽奖后还能参与抽奖的人员，d中奖人员
}
