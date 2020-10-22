use super::client::SeqClient;
use std::fmt::Debug;
use alsa::seq;

pub trait Transformer {
    fn parse_args(&mut self, args: Vec<String>);
    fn transform(&mut self, event: &mut seq::Event, seq: &SeqClient);
}

impl Debug for dyn Transformer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "dyn Transformer")
    }
}
