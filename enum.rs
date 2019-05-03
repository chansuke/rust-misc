#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum Residents {
    Chom, Chansuke, Kimurabro, Miminashi,
}

fn call_residents(residents: Residents) {
    if residents == Residents::Chom {
        println!("Hi, guchom!!!!!!!!");
    } else {
        println!("誰 {:?}か ", residents);
    }
}

fn main() {
    call_residents(Residents::Chom);
}
