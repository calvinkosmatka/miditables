use crate::matcher::Matcher;
use alsa::seq::{self, EventType};
use std::collections::HashSet;

pub struct EvTypeMatcher {
    types_to_match: HashSet<EventType>,
}

impl Matcher for EvTypeMatcher {
    fn parse_args(&mut self, args: Vec<String>) {
        for evtype in args.iter() {
            match evtype.as_ref() {
                "noteon" => {
                    self.types_to_match.insert(EventType::Noteon);
                }
                "noteoff" => {
                    self.types_to_match.insert(EventType::Noteoff);
                }
                "note" => {
                    self.types_to_match.insert(EventType::Note);
                }
                x => {
                    println!("evtypematcher {}", x);
                }
            }
        }
    }
    fn r#match(&mut self, event: &seq::Event) -> bool {
        self.types_to_match.contains(&event.get_type())
    }
}

impl EvTypeMatcher {
    pub fn new() -> EvTypeMatcher {
        EvTypeMatcher {
            types_to_match: HashSet::new(),
        }
    }
}
