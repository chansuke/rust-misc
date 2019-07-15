//types
struct Residents {
    name: String,
    gender: String,
}

struct Worker {
    name: String,
    gender: String,
}

trait Summary {
    fn summarize(self) -> String;
    //fn is_member(&self) -> Boolean;
}

impl Summary for Residents {
    fn summarize(self) -> String {
        format!("@{}", self.name)
    }
}

//impl Summary for Worker {
//    fn summarize(self) -> String {
//        println!("Gender is {}", self.gender)
//    }
//}

fn main() {
    let member = Residents {
        name: String::from("John"),
        gender: String::from("man"),
    };

    let name = member.summarize();

    println!("{}", name)
}
