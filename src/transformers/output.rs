use crate::ThreadOutput;
use crate::Transformer;
use alsa::seq;

pub struct OutputTransformer {
    
}

impl Transformer for OutputTransformer {
    fn parse_args(&mut self, args: Vec<String>) {

    }
    fn transform(&mut self, event: &mut seq::Event, seq: &ThreadOutput) {
        seq.send(event.clone().into_owned());
    }
}

impl OutputTransformer {
    pub fn new() -> OutputTransformer {
        OutputTransformer {}
    }
}
