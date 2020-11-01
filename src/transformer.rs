use super::ThreadOutput;
use std::fmt::Debug;
use alsa::seq;

pub trait Transformer: Send {
    fn parse_args(&mut self, args: Vec<String>);
    fn transform(&mut self, event: &mut seq::Event, seq: &ThreadOutput);
}

impl Debug for dyn Transformer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "dyn Transformer")
    }
}
