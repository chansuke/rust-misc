use std::cell::RefCell;
use std::collections::HashSet;

//https://doc.rust-lang.org/std/macro.thread_local.html
// thread_local macro doesn't support macro
thread_local!(
    static JAZZ: RefCell<HashSet<&'static str>> = {
        let rb = ["bebop", "piano"].iter().cloned().collect();
        RefCell::new(rb)
    }
);

fn main() {
    JAZZ.with(|jz| {
        assert!(jz.borrow().contains("bebop"));
        jz.borrow_mut().insert("guitor");
    });

    std::thread::spawn(||
        JAZZ.with(|jz| jz.borrow_mut().insert("miles"))
    ).join().expect("error");

    JAZZ.with(|jz| {
        assert!(jz.borrow().contains("bebop"));
    });
}
