#[derive(Debug)]
struct Child(usize);

impl Drop for Child {
    fn drop(&mut self) {
        println!("Dopping {:?}", self);
    }
}

use std::rc::Rc;

fn main() {
    // put the Child value int heap memory
    let mut rc1 = Rc::new(Child(1));

    // `strong_count はChild値の参照カウントを得る`
    println!("(a) count: {}, rc1: {:?}", Rc::strong_count(&rc1), rc1);
    {
        // cloneは共同所有者を作る。
        let rc2 = Rc::clone(&rc1);
        let rc3 = Rc::clone(&rc1);
        println!(
            "(b) count: {}, rc1: {:?}, rc2: {:?}, rc3: {:?}",
            // 参照カウントが増えているか
            Rc::strong_count(&rc1), rc1, rc2, rc3,
        );
    }
    // get out of scope and then the count is 0
    println!("(c) count: {}, rc1: {:?}", Rc::strong_count(&rc1), rc1);

    if let Some(child) = Rc::get_mut(&mut rc1) {
        child.0 += 1;
    }

    println!("(d) count: {}, rc1: {:?}", Rc::strong_count(&rc1), rc1);

    let weak = Rc::downgrade(&rc1);
    println!(
        "(e) count: {}, rc1: {:?}, weak: {:?}",
        Rc::strong_count(&rc1),
        rc1,
        weak,
    );

    if let Some(rc3) = weak.upgrade() {
        println!(
            "(f) count: {}, rc1: {:?}, rc3: {:?}",
            Rc::strong_count(&rc1),
            rc1,
            rc3,
        );
    }

    std::mem::drop(rc1);
    println!("(g) count: 0, weak.upgrade(): {:?}", weak.upgrade());
}
