mod debug;
mod note;
mod operations;
mod output;

use self::debug::DebugTransformer;
use self::note::NoteTransformer;
pub use self::operations::Operation;
use self::output::OutputTransformer;
use crate::Transformer;

pub fn get_plugin_transformers() -> Vec<String> {
    vec!["debug".to_string(), "output".to_string()]
}
pub fn get_transformer(transformer_name: &str) -> Option<Box<dyn Transformer>> {
    match transformer_name {
        "debug" => Some(Box::new(DebugTransformer::new())),
        "output" => Some(Box::new(OutputTransformer::new())),
        "note" => Some(Box::new(NoteTransformer::new())),
        _ => None,
    }
}
