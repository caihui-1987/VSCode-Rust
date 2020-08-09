mod factory {
    //生产冰箱
    pub mod produce_refrigerator {
        pub fn produce_re() {
            println!("produce refrigerator!");
        }
    }
    //生产洗衣机
    mod produce_washing_machine {
        fn produce_washing() {
            println!("produce washing machine!")
        }
    }
}
fn main() {
    factory::produce_refrigerator::produce_re();
    println!("Hello, world!");
}
