use proc_macro2::{Punct, Spacing, Span, TokenStream};
use quote::{ToTokens, TokenStreamExt};
use std::fmt::{self, Display};
use syn::{Ident, Path};

pub struct Interpolated(pub String);

impl ToTokens for Interpolated {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Punct::new('#', Spacing::Alone));
        tokens.append(Ident::new(self.0.as_str(), Span::call_site()));
    }
}

#[derive(Copy, Clone)]
pub struct Symbol(&'static str);

impl Symbol {
    pub fn new(path: &'static str) -> Self {
        Symbol(path)
    }
}

impl PartialEq<Symbol> for Ident {
    fn eq(&self, word: &Symbol) -> bool {
        self == word.0
    }
}

impl<'a> PartialEq<Symbol> for &'a Ident {
    fn eq(&self, word: &Symbol) -> bool {
        *self == word.0
    }
}

impl PartialEq<Symbol> for Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

impl<'a> PartialEq<Symbol> for &'a Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

impl Display for Symbol {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(self.0)
    }
}

pub const MOD_PATH: Symbol = Symbol("mod_path");
