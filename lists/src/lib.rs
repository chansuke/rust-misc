#![feature(uniform_paths, test)]

mod doubly_linked_list;
mod dynamic_array;
mod singly_linked_list;
mod skip_list;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
