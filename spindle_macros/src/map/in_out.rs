#[derive(Debug, Clone)]
pub(crate) struct InOut {
    pub(crate) input: Option<syn::Ident>,
    pub(crate) output: Option<syn::Ident>,
}
