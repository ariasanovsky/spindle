use proc_macro2::TokenStream;

pub(super) trait NaivelyTokenize: Sized + ToString {
    fn naively_tokenize(self) -> TokenStream {
        let s = self.to_string();
        println!("{s}");
        quote::quote! { #s }
        // panic!("{}", self.to_string());
    }
}

impl NaivelyTokenize for std::io::Error {}
impl NaivelyTokenize for serde_json::Error {}
impl NaivelyTokenize for String {
    fn naively_tokenize(self) -> TokenStream {
        println!("{self}");
        // quote::quote! { #self } todo! sanitize self
        quote::quote! { "kernel compile error: see terminal" }
    }
}

pub(super) fn command_output_result(output: std::process::Output) -> Result<String, String> {
    let msg = match (&output.stderr, &output.stdout) {
        (err, out) if !err.is_empty() && !out.is_empty() => {
            format!(
                "{}\n{}",
                String::from_utf8_lossy(err).trim_end(),
                String::from_utf8_lossy(out).trim_start()
            )
        }
        (err, _) if !err.is_empty() => String::from_utf8_lossy(err).to_string(),
        (_, out) if !out.is_empty() => String::from_utf8_lossy(out).to_string(),
        _ => String::from_utf8_lossy(output.status.to_string().as_bytes()).to_string(),
    };
    if output.status.success() {
        Ok(msg)
    } else {
        Err(msg)
    }
}
