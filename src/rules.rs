use super::matcher::Matcher;
use super::transformer::Transformer;
use super::ThreadOutput;
use alsa::seq;
use std::sync::Mutex;

pub struct Rule {
    matchers: Vec<Mutex<Box<dyn Matcher>>>,
    transformers: Vec<Mutex<Box<dyn Transformer>>>,
    jump: Jump,
}

impl Rule {
    pub fn new(matchers: Vec<Mutex<Box<dyn Matcher>>>, transformers: Vec<Mutex<Box<dyn Transformer>>>, jump: Jump) -> Rule {
        Rule {
            matchers,
            transformers,
            jump
        }
    }
    pub fn process(&self, event: &mut seq::Event, seq: &ThreadOutput) -> Jump {
        for matcher in self.matchers.iter() {
            if !matcher.lock().unwrap().r#match(event) {
                return Jump::Continue;
            }
        }
        for transformer in self.transformers.iter() {
            transformer.lock().unwrap().transform(event, seq);
        }
        self.jump.clone()
    }
}

#[derive(Clone)]
#[derive(Debug)]
pub enum Jump {
    Continue,
    End,
    Chain(String),
}
