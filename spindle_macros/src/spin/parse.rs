use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};

use crate::{case::{UpperCamelIdent, PrimitiveIdent, LowerSnakeIdent, Cased}, union::{MapFnInScope, NewUnion, UnionInScope}, tag::CrateTag};

use super::{RawSpinInput, RawSpinInputs, UnionInput, SpinInputs};

impl Parse for SpinInputs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // e.g., `#example_01, U = i32 | f64`
        // always begin with exactly one crate tag
        let tag: CrateTag = input.parse()?;
        // consume a comma
        let _: syn::Token![,] = input.parse()?;
        // consume a union (there is always at least one)
        let u: UnionInput = input.parse()?;
        let mut unions: Vec<UnionInput> = vec![u];
        while !input.is_empty() {
            // consume a comma
            let _: syn::Token![,] = input.parse()?;
            if input.is_empty() {
                break;
            }
            // consume a union
            let u: UnionInput = input.parse()?;
            unions.push(u);
        }
        Ok(SpinInputs {
            tag,
            unions,
        })
        // let err = syn::Error::new_spanned(
        //     input.cursor().token_stream(),
        //     format!("make SpinInputs from {input:#?}"),
        // );
        // Err(err)
    }
}

impl Parse for UnionInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // e.g., `U = i32 | f64`
        // e.g., `V`
        let ident: UpperCamelIdent = input.parse()?;
        // peek for an `=`
        Ok(if input.peek(syn::Token![=]) {
            // consume the `=`
            let _: syn::Token![=] = input.parse()?;
            // consume a primitive
            let p: PrimitiveIdent = input.parse()?;
            let mut fields: Vec<PrimitiveIdent> = vec![p];
            // while we see a `|`, consume it and a primitive
            while input.peek(syn::Token![|]) {
                // consume the `|`
                let _: syn::Token![|] = input.parse()?;
                // consume a primitive
                let p: PrimitiveIdent = input.parse()?;
                fields.push(p);
            }
            UnionInput::New(ident, fields)
        } else {
            UnionInput::InScope(ident)
        })
        // let err = syn::Error::new_spanned(
        //     input.cursor().token_stream(),
        //     format!("make UnionInput from {input:#?}"),
        // );
        // return Err(err);
    }
}

impl Parse for RawSpinInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        /* grammar:
            `U = p | q | ... | r` where `p`, `q`, ..., `r` are primitives
            `V` where `V` is a union in scope
            `foo` where `foo` is a map in scope
        */
        // first, parse the first ident
        let ident: Ident = input.parse()?;
        use crate::case::Case;
        let u = match ident.to_string().as_str().case() {
            Case::LowerSnake => return Ok(Self::MapFnInScope(MapFnInScope(LowerSnakeIdent(ident)))),
            Case::UpperCamel => ident,
            Case::SupportedPrimitive | Case::UnsupportedPrimitive => return Err(syn::Error::new_spanned(ident, "primitive types are not supported here")),
            Case::Unknown => return Err(syn::Error::new_spanned(ident, "expected a union (`U`), map (`foo`), or primitive (`f32`)")),
        };
        let ident = UpperCamelIdent(u);
        // dbg!(&ident);
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
            RawSpinInput::NewUnion(NewUnion(ident, fields))
        } else {
            // we have parsed `V` and expect nothing more
            RawSpinInput::UnionInScope(UnionInScope(ident))
        })
        // todo!()
    }
}

impl Parse for RawSpinInputs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // we expect exactly one crate tag, followed by a comma
        let crate_tag: CrateTag = input.parse()?;
        let _ = input.parse::<syn::Token![,]>()?;
        // we expect at least one comma-separated `RawSpinInput`s
        let spin_input: RawSpinInput = input.parse()?;
        let mut unions_in_scope = Vec::new();
        let mut new_unions = Vec::new();
        let mut map_fns_in_scope = Vec::new();
        match spin_input {
            RawSpinInput::UnionInScope(u) => unions_in_scope.push(u),
            RawSpinInput::NewUnion(u) => new_unions.push(u),
            RawSpinInput::MapFnInScope(f) => map_fns_in_scope.push(f),
        }
        // now we have parsed `U = p | q | ... r` and did not check for a comma
        while input.peek(syn::Token![,]) {
            // consume the `,` token and parse the next `RawSpinInput`
            let _ = input.parse::<syn::Token![,]>()?;
            // let's allow a trailing comma
            if input.is_empty() {
                break;
            }
            let spin_input: RawSpinInput = input.parse()?;
            match spin_input {
                RawSpinInput::UnionInScope(u) => unions_in_scope.push(u),
                RawSpinInput::NewUnion(u) => new_unions.push(u),
                RawSpinInput::MapFnInScope(f) => map_fns_in_scope.push(f),
            }
        }
        Ok(RawSpinInputs {
            _crate_tag: crate_tag,
            _unions_in_scope: unions_in_scope,
            _new_unions: new_unions,
            _map_fns_in_scope: map_fns_in_scope,
        })
    }
}