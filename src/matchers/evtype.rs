use crate::matcher::Matcher;
use alsa::seq;

pub struct EvTypeMatcher {

}

impl Matcher for EvTypeMatcher {
    fn parse_args(&mut self, args: Vec<String>) {

    }
    fn r#match(&mut self, event: &seq::Event) -> bool {
        true
    }
}

impl EvTypeMatcher {
    pub fn new() -> EvTypeMatcher {
        EvTypeMatcher {}
    }
}
