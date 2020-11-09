use super::chain::Chain;
use super::rules::Jump;
use super::ThreadOutput;
use alsa::seq;
use std::collections::HashMap;

pub struct Table {
    default_chain: Chain,
    chains: HashMap<String, Chain>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            default_chain: Chain::new(),
            chains: HashMap::new(),
        }
    }
    pub fn get_chain(&self, chain_name: &str) -> Option<&Chain> {
        if chain_name == "default" {
            return Some(&self.default_chain);
        }
        self.chains.get(chain_name)
    }
    pub fn get_chain_mut(&mut self, chain_name: &str) -> Option<&mut Chain> {
        if chain_name == "default" {
            return Some(&mut self.default_chain);
        }
        self.chains.get_mut(chain_name)
    }
    pub fn add_chain(&mut self, chain_name: &str, chain: Chain) {
        // do some checking about chain names here
        if self.chains.contains_key(chain_name) && chain_name != "default" {
            panic!("chain already exists");
        } else {
            self.chains.insert(chain_name.to_string(), chain);
        }
    }
    pub fn process(&self, event: &mut seq::Event, seq: ThreadOutput) {
        let mut jump = self.default_chain.process(event, &seq);
        loop {
            jump = match jump {
                Jump::Continue => {
                    // a chain really shouldn't return this
                    break;
                }
                Jump::End => {
                    break;
                }
                Jump::Chain(chain) => self.chains.get(&chain).unwrap().process(event, &seq),
            };
        }
    }
}
