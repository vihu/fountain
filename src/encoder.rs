use crate::droplet::Droplet;

pub trait Encoder {
    fn next(&mut self) -> Droplet;
}
