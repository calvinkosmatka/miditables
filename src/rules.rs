use super::matcher::Matcher;
use super::transformer::Transformer;
use super::client::SeqClient;
use alsa::seq;

pub struct Rule {
    matchers: Vec<Box<dyn Matcher>>,
    transformers: Vec<Box<dyn Transformer>>,
    jump: Jump,
}

impl Rule {
    pub fn new(matchers: Vec<Box<dyn Matcher>>, transformers: Vec<Box<dyn Transformer>>, jump: Jump) -> Rule {
        Rule {
            matchers,
            transformers,
            jump
        }
    }
    pub fn process(&mut self, event: &mut seq::Event, seq: &SeqClient) -> Jump {
        for matcher in self.matchers.iter_mut() {
            if !matcher.r#match(event) {
                return Jump::Continue;
            }
        }
        for transformer in self.transformers.iter_mut() {
            transformer.transform(event, seq);
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
