extern crate alsa;
use std::sync::mpsc::Sender;

#[macro_use]
pub mod plugin;
pub use plugin::{PluginGetMatcherFn,PluginGetTransformerFn};
pub mod tables;
pub mod chain;
pub mod rules;
pub mod matcher;
pub use matcher::Matcher;
pub mod transformer;
pub use transformer::Transformer;
pub mod matchers;
pub mod transformers;
pub mod config;
pub mod client;

pub type ThreadOutput<'a> = Sender<alsa::seq::Event<'a>>;
