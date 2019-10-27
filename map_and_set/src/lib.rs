#![feature(test, bind_by_move_pattern_guards)]

mod map;
mod set;

#[derive(Clone, Debug, PartialEq)]
pub struct OfficialMember {
    id: u64,
    name: String,
    address: String,
    description: String
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
