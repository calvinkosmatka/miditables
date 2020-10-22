mod debug;

use self::debug::DebugTransformer;
use crate::Transformer;

register_transformers! { "debug" => DebugTransformer }
