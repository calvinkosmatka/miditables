mod evtype;
mod inport;

use self::evtype::EvTypeMatcher;
use self::inport::InPortMatcher;
use crate::Matcher;

register_matchers! { "evtype" => EvTypeMatcher , "port" => InPortMatcher}

