// use std::cell::RefCell;
// use std::rc::Rc;
use crate::droplet::RxDroplet;

#[derive(Clone)]
pub struct Block {
    pub idx: usize,
    pub edges: Vec<RxDroplet>,
    pub begin_at: usize,
    pub is_known: bool,
}

impl Block {
    pub fn new(
        idx: usize,
        edges: Vec<RxDroplet>,
        begin_at: usize,
        is_known: bool ) -> Block {
        Block {
            idx,
            edges,
            begin_at,
            is_known
        }
    }
}
