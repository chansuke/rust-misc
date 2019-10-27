use crate::IoTDevice;
use std::cell::{Ref, RefCell};
use std::cmp;
use std::mem;
use std::rc::Rc;

type BareTree = Rc<RefCell<Node>>;
type Tree = Option<BareTree>;

#[derive(Clone, Debug, PartialEq)]
enum Color {
    Red,
    Black,
}


