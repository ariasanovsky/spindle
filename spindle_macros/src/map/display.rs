use std::fmt::{Debug, Display};

use quote::ToTokens;

use super::{MapFn, in_out::InOut};

impl Debug for MapFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MapFn")
        .field("item_fn", &self.item_fn.clone().to_token_stream().to_string())
        .field("in_outs", &self.in_outs.iter().map(|in_out| in_out.to_string()).collect::<Vec<_>>())
        .finish()
    }
}

impl Display for InOut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InOut")
        .field("input", &self.input.as_ref().map_or("_".to_string(), |input| input.to_string()))
        .field("output", &self.output.as_ref().map_or("_".to_string(), |output| output.to_string()))
        .finish()
    }
}
