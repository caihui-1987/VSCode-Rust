fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut larger = list[0];
    for &item in list.iter() {
        if item > larger {
            larger = item;
        }
    }
    larger
}
fn main() {
    let number_list = vec![1, 2, 32, 34, 8, 100];
    let max_number = largest(&number_list);
    println!("max_number = {}", max_number);

    let char_list = vec!['a', 't', 'b'];
    let max_char = largest(&char_list);
    println!("max_char = {}", max_char);
    println!("Hello, world!");
}
