use std::str::FromStr;
use crate::ThreadOutput;
use crate::Transformer;
use alsa::seq;

pub struct OutputTransformer {
    port: Option<u8>    
}

impl Transformer for OutputTransformer {
    fn parse_args(&mut self, args: Vec<String>) {
        self.port = match args.get(0) {
            Some(x) => {
                Some(u8::from_str(x).expect("not a u8"))
            },
            None => { None },
        };
    }
    fn transform(&mut self, event: &mut seq::Event, seq: &ThreadOutput) {
        if let Some(port) = self.port {
            event.set_source(port as i32);
        }
        seq.send(event.clone().into_owned());
    }
}

impl OutputTransformer {
    pub fn new() -> OutputTransformer {
        OutputTransformer {
            port: None
        }
    }
}
