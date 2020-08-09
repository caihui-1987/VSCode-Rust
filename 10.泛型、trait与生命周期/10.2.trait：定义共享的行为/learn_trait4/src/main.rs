//使用trait bound 有条件的实现方法
//定义两个类型
trait GetName {
    fn get_name(&self) -> &String;
}
trait GetAge {
    fn get_age(&self) -> u32;
}
//定义一个结构体 
struct PeopleMatchInformation<T, U> {
    master: T,
    student: U,
}
//定义该结构体方法，使用泛型
impl<T: GetName + GetAge, U: GetName + GetAge> PeopleMatchInformation<T, U> {
    fn print_information(&self) {
        println!("master name ={}", self.master.get_name());
        println!("master age ={}", self.master.get_age());
        println!("student name ={}", self.student.get_name());
        println!("student age ={}", self.student.get_age());
    }
}
//定义Teacher结构体
struct Teacher {
    name: String,
    age: u32,
}
impl GetName for Teacher {
    fn get_name(&self) -> &String {
        &(self.name)
    }
}
impl GetAge for Teacher {
    fn get_age(&self) -> u32 {
        self.age
    }
}
//定义Student  结构体
struct Student {
    name: String,
    age: u32,
}
impl GetName for Student {
    fn get_name(&self) -> &String {
        &(self.name)
    }
}
impl GetAge for Student {
    fn get_age(&self) -> u32 {
        self.age
    }
}
fn main() {
    let s = Student {
        name: "xiaoming".to_string(),
        age: 15,
    };
    let t = Teacher {
        name: "xiaohuang".to_string(),
        age: 30,
    };
    let m = PeopleMatchInformation {
        master: t,
        student: s,
    };
    m.print_information();
}
