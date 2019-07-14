use toyvec::ToyVec;

fn main() {
    let mut v = ToyVec::new();
    v.push("Taro".to_string());
    v.push("Jiro".to_string());

    let mut iter = v.iter();

    //v.push("saburo".to_string()); // should be error since `fn next(&mut self) returns (&T)`

    assert_eq!(iter.next(), Some(&"Jiro".to_string()));

    v.push("Shiro".to_string());
}
