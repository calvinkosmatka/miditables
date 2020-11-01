use super::rules::{Rule, Jump};
use super::ThreadOutput;
use alsa::seq;

pub struct Chain {
    pub rules: Vec<Rule>,
}

impl Chain {
    pub fn new() -> Chain {
        Chain {
            rules: Vec::new(),
        }
    }
    pub fn process(&self, event: &mut seq::Event, seq: &ThreadOutput) -> Jump {
        for rule in self.rules.iter() {
            let j = rule.process(event, seq);
            if let Jump::Continue = j {
                continue;
            } else {
                return j;
            }
        }
        Jump::End
    }
}
