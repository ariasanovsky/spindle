use proc_macro2::Span;
use quote::ToTokens;
use spindle_db::{map::DbMap, TypeDb};

use crate::{
    case::{PrimitiveIdent, UpperCamelIdent},
    tag::CrateTag,
    union::{MapFnInScope, NewUnion, RawSpinInput, UnionInScope}, spin::compile::CompilationStatus,
};

mod compile;
mod error;
mod parse;
mod spindle_crate;
#[cfg(test)]
mod test;
mod tokens;

#[derive(Debug)]
pub struct SpinInputs {
    pub tag: CrateTag,
    pub unions: Vec<UnionInput>,
}

#[derive(Debug)]
pub enum UnionInput {
    New(UpperCamelIdent, Vec<PrimitiveIdent>),
    InScope(UpperCamelIdent),
}

#[derive(Debug)]
pub struct SpindleCrate {
    pub home: std::path::PathBuf,
    pub maps: Vec<DbMap>,
    pub tag: CrateTag,
    pub unions: Vec<UnionInput>,
}

pub fn spin(inputs: SpinInputs, db_name: &str) -> syn::Result<proc_macro2::TokenStream> {
    // map any conversion errors to the call site
    let spindle_crate: SpindleCrate = (inputs, db_name)
        .try_into()
        .map_err(|err| syn::Error::new(Span::call_site(), format!("{err:#?}")))?;
    let status: CompilationStatus = spindle_crate.status()?;
    status.print_colorful_output();
    match status {
        CompilationStatus::Failed { colorless_output, .. } => {
            // return the error
            Err(syn::Error::new(Span::call_site(), colorless_output))
        },
        CompilationStatus::Succeeded { .. } => {
            // return the crate as a token stream
            Ok(spindle_crate.into_token_stream())
        },
    }
}

impl TryFrom<(SpinInputs, &str)> for SpindleCrate {
    type Error = spindle_db::Error;

    fn try_from((inputs, db_name): (SpinInputs, &str)) -> Result<Self, Self::Error> {
        let SpinInputs { tag, unions } = inputs;
        let target = std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
        let db = format!("{target}/spindle/db/");
        let db = TypeDb::open_or_create(db_name, db)?;
        let maps = db.get_maps_from_tag(&tag)?;
        let home = format!("{target}/spindle/map/");
        let home = std::path::PathBuf::from(home).join(tag.to_string());
        Ok(Self {
            home,
            maps,
            tag,
            unions,
        })
    }
}

#[derive(Debug)]
pub(crate) struct RawSpinInputs {
    pub _crate_tag: CrateTag,
    pub _unions_in_scope: Vec<UnionInScope>,
    pub _new_unions: Vec<NewUnion>,
    pub _map_fns_in_scope: Vec<MapFnInScope>,
}
