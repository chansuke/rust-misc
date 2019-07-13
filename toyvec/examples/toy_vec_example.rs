use toyvec::ToyVec;

fn main() {
    let e: Option<&String>;
    {
        // スコープ開始
        let mut v = ToyVec::new();
        v.push("kendrick".to_string());
        v.push("john".to_string());
    
        // error
        e = v.get(0);
        // スコープ終了
    }
    
    assert_eq!(e, Some(&"kendrick".to_string()));
    assert_eq!(e, Some(&"eminem".to_string()));
}
