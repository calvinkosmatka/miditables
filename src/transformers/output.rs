use crate::client::SeqClient;
use Transformer;
use alsa::seq;

pub struct OutputTransformer {
    
};

impl Transformer for OutputTransformer {
    fn parse_args(&mut self, args: Vec<String>) {

    }
    fn transform(&mut self, event: &mut seq::Event, seq: &SeqClient) {
        
        ev.set_subs();
        ev.set_direct();
        seq.event_output(event);
    }
}
