use std::cell::RefCell;
use std::rc::Rc;
use crate::droplet::RxDroplet;

type Edges = Vec<Rc<RefCell<RxDroplet>>>;

pub struct Block {
    pub idx: usize,
    pub edges: Edges,
    pub begin_at: usize,
    pub is_known: bool,
}

impl Block {
    pub fn new(
        idx: usize,
        edges: Edges,
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
