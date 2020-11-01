use crate::{Matcher, Transformer};

pub type PluginGetMatcherFn = fn(&str) -> Option<Box<dyn Matcher>>;
pub type PluginGetTransformerFn = fn(&str) -> Option<Box<dyn Transformer>>;

#[macro_export]
macro_rules! register_matchers {
    ($($matcher_name:literal => $matcher_type:ty),+) => {
        #[no_mangle]
        pub extern fn get_plugin_matchers() -> Vec<String> {
            vec! [ $( $matcher_name.to_string() ),+ ]
        }
        #[no_mangle]
        pub extern fn get_matcher(matcher_name: &str) -> Option<Box<dyn Matcher>> {
            match matcher_name {
                $( $matcher_name => Some(Box::new(<$matcher_type>::new())), )+
                _ => None,
            }
        }
    };
}

#[macro_export]
macro_rules! register_transformers {
    ($($transformer_name:literal => $transformer_type:ty),+) => {
        #[no_mangle]
        pub extern fn get_plugin_transformers() -> Vec<String> {
            vec! [ $( $transformer_name.to_string() ),+ ]
        }
        #[no_mangle]
        pub extern fn get_transformer(transformer_name: &str) -> Option<Box<dyn Transformer>> {
            match transformer_name {
                $( $transformer_name => Some(Box::new(<$transformer_type>::new())), )+
                _ => None,
            }
        }
    };
}
