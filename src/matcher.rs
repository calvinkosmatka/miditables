use std::fmt::Debug;
use alsa::seq;

pub trait Matcher {
    fn parse_args(&mut self, args: Vec<String>);
    fn r#match(&mut self, event: &seq::Event) -> bool;
}

impl Debug for dyn Matcher {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "dyn Matcher")
    }
}

