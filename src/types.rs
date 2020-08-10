use crate::decoder::Statistics;

#[derive(Clone, Debug)]
pub enum EncoderType {
    /// The first k symbols of a systematic Encoder correspond to the first k source symbols
    /// In case there is no loss, no repair needed. After the first k symbols are sent, it continous
    /// like in the Random case.
    Systematic,
    /// Begins immediately with random encoding.
    /// This may be a better choice when used with high-loss channels.
    Random,
}

#[derive(Debug)]
pub enum DropType {
    /// First is seed, second degree
    Seeded(u64, usize),
    /// Just a list of edges
    Edges(usize),
}

#[derive(Debug)]
pub enum CatchResult {
    Finished(Vec<u8>, Statistics),
    Missing(Statistics),
}
