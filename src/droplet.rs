use crate::types::DropType;

/// A Droplet is created by the Encoder.
#[derive(Debug)]
pub struct Droplet {
    /// The droptype can be based on seed or a list of edges
    pub droptype: DropType,
    /// The payload of the Droplet
    pub data: Vec<u8>,
}

impl Droplet {
    pub fn new(droptype: DropType, data: Vec<u8>) -> Droplet {
        Droplet { droptype, data }
    }
}

#[derive(Debug, Clone)]
pub struct RxDroplet {
    pub edges_idx: Vec<usize>,
    pub data: Vec<u8>,
}
