use super::ThreadOutput;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use alsa::seq;

pub enum TransformerType {
    LocalTransformer(Mutex<Box<dyn Transformer>>),
    GlobalTransformer(Arc<Mutex<Box<dyn Transformer>>>),
}

pub trait Transformer: Send {
    fn parse_args(&mut self, args: Vec<String>);
    fn transform(&mut self, event: &mut seq::Event, seq: &ThreadOutput);
}

impl Debug for dyn Transformer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "dyn Transformer")
    }
}
