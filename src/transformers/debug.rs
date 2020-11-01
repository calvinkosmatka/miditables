use crate::ThreadOutput;
use crate::Transformer;
use alsa::seq;

pub struct DebugTransformer;

impl Transformer for DebugTransformer {
    fn parse_args(&mut self, _args: Vec<String>) {

    }
    fn transform(&mut self, event: &mut seq::Event, _seq: &ThreadOutput) {
        println!("Received: {:?}", event);
    }
}

impl DebugTransformer {
    pub fn new() -> DebugTransformer {
        DebugTransformer
    }
}
