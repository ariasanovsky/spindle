use proc_macro2::Span;
use quote::ToTokens;
use spindle_db::{map::DbMap, TypeDb};

use crate::{
    case::{PrimitiveIdent, UpperCamelIdent},
    tag::CrateTag,
    union::{MapFnInScope, NewUnion, RawSpinInput, UnionInScope},
};

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
    if !spindle_crate.exists() {
        spindle_crate.populate()?;
        let output: std::process::Output = spindle_crate.compile()?;
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !stdout.is_empty() {
            println!("{stdout}");
        }
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        if !stderr.is_empty() {
            println!("{stderr}");
        }
    }
    // let _: () = spindle_crate.write_toml_files().map_err(|err| syn::Error::new(
    //     Span::call_site(),
    //     format!("{err:#?}"),
    // ))?;
    // let _: () = spindle_crate.write_lib_rs().map_err(|err| syn::Error::new(
    //     Span::call_site(),
    //     format!("{err:#?}"),
    // ))?;
    // let _: () = spindle_crate.write_device_rs().map_err(|err| syn::Error::new(
    //     Span::call_site(),
    //     format!("{err:#?}"),
    // ))?;
    Ok(spindle_crate.to_token_stream())
}

impl TryFrom<(SpinInputs, &str)> for SpindleCrate {
    type Error = spindle_db::Error;

    fn try_from((inputs, db_name): (SpinInputs, &str)) -> Result<Self, Self::Error> {
        let SpinInputs { tag, unions } = inputs;
        let db = TypeDb::open_or_create(db_name, "target/spindle/db/")?;
        let maps = db.get_maps_from_tag(&tag)?;
        let home = std::path::PathBuf::from("target/spindle/crates/").join(tag.to_string());
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
