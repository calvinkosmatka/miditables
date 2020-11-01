mod evtype;
mod inport;

use self::evtype::EvTypeMatcher;
use self::inport::InPortMatcher;
use crate::Matcher;

pub fn get_plugin_matchers() -> Vec<String> {
    vec! [ "evtype".to_string(),
        "port".to_string()
    ]
}
pub fn get_matcher(matcher_name: &str) -> Option<Box<dyn Matcher>> {
    match matcher_name {
        "evtype" => Some(Box::new(EvTypeMatcher::new())),
        "port" => Some(Box::new(InPortMatcher::new())),
        _ => None,
    }
}
