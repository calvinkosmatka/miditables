use super::rules::{Rule, Jump};
use super::client::SeqClient;
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
    pub fn process(&mut self, event: &mut seq::Event, seq: &SeqClient) -> Jump {
        for rule in self.rules.iter_mut() {
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
