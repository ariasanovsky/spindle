use proc_macro2::Ident;

use crate::camel_word;

use super::{SNAKE_NAME_HEAD, SNAKE_NAME_TAIL};

pub(crate) trait RegulateIdent: Sized {
    fn at_most_one_leading_underscore(self) -> Result<Self, &'static str>;
    fn no_trailing_underscores(self) -> Result<Self, &'static str>;
    fn trimmed_lower_snake_to_trimmed_upper_camel(self) -> Result<(Self, Self), &'static str>;
    // fn trimmed_upper_camel_to_trimmed_lower_snake(self) -> Result<(Self, Self), &'static str>;
    // eventually for structs, enums, etc.
}

impl RegulateIdent for Ident {
    fn at_most_one_leading_underscore(self) -> Result<Self, &'static str> {
        let name = self.to_string();
        if name.starts_with("__") {
            return Err(SNAKE_NAME_HEAD);
        }
        Ok(self)
    }

    fn no_trailing_underscores(self) -> Result<Self, &'static str> {
        let name = self.to_string();
        if name.ends_with('_') {
            return Err(SNAKE_NAME_TAIL);
        }
        Ok(self)
    }

    fn trimmed_lower_snake_to_trimmed_upper_camel(self) -> Result<(Self, Self), &'static str> {
        let name = self.to_string();
        let words = name.split('_');
        let camel_words = words.map(camel_word).collect::<Vec<_>>();
        let camel = camel_words.join("");
        let camel = Ident::new(&camel, self.span());
        Ok((self, camel))
    }
}
