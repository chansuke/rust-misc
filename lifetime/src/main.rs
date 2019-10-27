fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// you only need one annotation if return value is only one
//fn longest<'a>(x: &'a str, y: str) -> &'a str {
//    x
//}

// if you create a struct with reference, you will also need lifetime annotation
struct Shortest<'a> {
    part: &'a str,
}

fn main() {
    let string1 = String::from("hogehoge");
    let string2 = String::from("nyaos");

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);

    // or let mut names = Vec::new() if you want to add something later
    let names = vec!["kate", "johan", "kendrick", "jason"];
    let name = names[0];

    let s = Shortest { part: name };
}
