//struct Human<T, U> {
//    name: T,
//    age: U,
//}
//
//fn main() {
//    let _hoge = Human { name: "joan", age: 13 };
//    println!("My name is {} and {} years old", _hoge.name, _hoge.age);
//}
//
struct Human<T> {
    first_name: T,
    last_name: T,
}

impl<T> Human<T> {
    fn name(&self) -> &T {
        &self.first_name
    }
}

fn main() {
    let myself = Human { first_name: "Kendrick", last_name: "Lamar" };

    println!("My name is {}", myself.first_name);
}
