use quote::ToTokens;
use syn::parse::Parse;

use crate::case::{UpperCamelIdent, PrimitiveIdent};

use super::RawSpinInput;

impl Parse for RawSpinInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        /* grammar:
            `U = p | q | ... | r` where `p`, `q`, ..., `r` are primitives
            `V` where `V` is a union in scope
        */
        let ident: UpperCamelIdent = input.parse()?;
        dbg!(&ident);
        // todo! check that `ident` is not a reserved word
        // peek the next token to see if it is an `=`
        let is_new_union = input.peek(syn::Token![=]);
        Ok(if is_new_union {
            // consume to the `=` token
            let _ = input.parse::<syn::Token![=]>()?;
            // we have parsed `U =` and expected `p | q | ...`
            // this terminates with a `,` or at the end of the input
            // we expect at least one primitive
            let field: PrimitiveIdent = input.parse()?;
            let mut fields: Vec<PrimitiveIdent> = vec![field];
            // now we have parsed `U = p` and expect `| q` some number of times
            while input.peek(syn::Token![|]) {
                // consume the `|` token and parse the next primitive
                let _ = input.parse::<syn::Token![|]>()?;
                let field: PrimitiveIdent = input.parse()?;
                fields.push(field);
            }
            // now we have parsed `U = p | q | ... r` and did not find another `|`
            RawSpinInput::NewUnion(ident, fields)
        } else {
            // we have parsed `V` and expect nothing more
            RawSpinInput::OldUnion(ident)
        })
    }
}