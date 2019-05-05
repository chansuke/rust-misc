#[derive(Debug)]
struct Parent(usize, Child, Child);

use std::ops::Drop;

impl Drop for Parent {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

#[derive(Debug)]
struct Child(usize);

impl Drop for Child {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

fn main() {
    println!("Move semantics!!");
    move_semantics();
    println!("\nBorrow: ");
    borrow();
}

fn move_semantics() {
    let mut p1 = Parent(1, Child(11), Child(12));
    let p2 = p1; //所有権の移転
    println!("p2: {:?}", p2);
    //println!("p1: {:?}", p1); //これはエラーになる

    p1 = Parent(2, Child(21), Child(22));
    println!("p1: {:?}", p1);
}

fn f1(p: &Parent) {
    println!("p: {:?}", p);
}

fn f2(p: &mut Parent) {
    p.0 *= 1;
}

fn borrow() {
    let mut p1 = Parent(1, Child(11), Child(12));
    f1(&p1);                   // f1には所有権をムーブせず、不変の参照を渡す
    f2(&mut p1);               // f2には所有権をムーブせず、可変の参照を渡す
    println!("p1: {:?}", p1); 
}
