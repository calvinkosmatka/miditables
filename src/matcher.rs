use alsa::seq;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

pub enum MatcherType {
    LocalMatcher(Mutex<Box<dyn Matcher>>),
    LocalMatcherNeg(Mutex<Box<dyn Matcher>>),
    GlobalMatcher(Arc<Mutex<Box<dyn Matcher>>>),
    GlobalMatcherNeg(Arc<Mutex<Box<dyn Matcher>>>),
}

pub trait Matcher: Send {
    fn parse_args(&mut self, args: Vec<String>);
    fn r#match(&mut self, event: &seq::Event) -> bool;
}

impl Debug for dyn Matcher {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "dyn Matcher")
    }
}
