use crate::Matcher;
use alsa::seq;
use std::str::FromStr;

pub struct InPortMatcher {
    in_port: u8,
}

impl Matcher for InPortMatcher {
    fn parse_args(&mut self, args: Vec<String>) {
        self.in_port = u8::from_str(&args[0]).expect("not a u8");
    }
    fn r#match(&mut self, event: &seq::Event) -> bool {
        event.get_dest().port == self.in_port as i32
    }
}

impl InPortMatcher {
    pub fn new() -> InPortMatcher {
        InPortMatcher { in_port: 0 }
    }
}
