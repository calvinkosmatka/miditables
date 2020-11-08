use super::matcher::{
    Matcher,
    MatcherType::{self, *},
};
use super::transformer::{
    Transformer,
    TransformerType::{self, *},
};
use super::ThreadOutput;
use alsa::seq;
use std::sync::Mutex;

pub struct Rule {
    matchers: Vec<MatcherType>,
    transformers: Vec<TransformerType>,
    jump: Jump,
}

impl Rule {
    pub fn new(matchers: Vec<MatcherType>, transformers: Vec<TransformerType>, jump: Jump) -> Rule {
        Rule {
            matchers,
            transformers,
            jump,
        }
    }
    pub fn process(&self, event: &mut seq::Event, seq: &ThreadOutput) -> Jump {
        for matcher in self.matchers.iter() {
            match matcher {
                LocalMatcher(m) => {
                    if !m.lock().unwrap().r#match(event) {
                        return Jump::Continue;
                    }
                }
                LocalMatcherNeg(m) => {
                    if m.lock().unwrap().r#match(event) {
                        return Jump::Continue;
                    }
                }
                GlobalMatcher(m) => {
                    if !m.lock().unwrap().r#match(event) {
                        return Jump::Continue;
                    }
                }
                GlobalMatcherNeg(m) => {
                    if m.lock().unwrap().r#match(event) {
                        return Jump::Continue;
                    }
                }
            }
        }
        for transformer in self.transformers.iter() {
            match transformer {
                LocalTransformer(t) => {
                    t.lock().unwrap().transform(event, seq);
                }
                GlobalTransformer(t) => {
                    t.lock().unwrap().transform(event, seq);
                }
            }
        }
        self.jump.clone()
    }
}

#[derive(Clone, Debug)]
pub enum Jump {
    Continue,
    End,
    Chain(String),
}
