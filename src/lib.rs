extern crate alsa;
use std::sync::mpsc::Sender;

#[macro_use]
pub mod plugin;
pub use plugin::{PluginGetMatcherFn, PluginGetTransformerFn};
pub mod chain;
pub mod matcher;
pub mod rules;
pub mod tables;
pub use matcher::Matcher;
pub mod transformer;
pub use transformer::Transformer;
pub mod client;
pub mod config;
pub mod matchers;
pub mod transformers;

pub enum SendEventType<'a> {
    Output(alsa::seq::Event<'a>),
}

pub type ThreadOutput<'a> = Sender<SendEventType<'a>>;
