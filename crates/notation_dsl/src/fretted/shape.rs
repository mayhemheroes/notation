use fehler::{throw, throws};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, ParseStream};
use syn::{LitInt, Token};

use crate::context::Context;
use crate::core::duration::DurationTweakDsl;

pub struct ShapeDsl {
    pub frets: Vec<Option<u8>>,
    pub duration_tweak: Option<DurationTweakDsl>,
}

impl ShapeDsl {
    #[throws(Error)]
    pub fn parse_without_paren(input: ParseStream, multied: bool, with_paren: bool) -> Self {
        let mut frets = vec![];
        if multied && !with_paren {
            throw!(Error::new(input.span(), "paren required in multied mode"));
        }
        while input.peek(LitInt) || input.peek(Token![_]) {
            if input.peek(LitInt) {
                frets.push(Some(input.parse::<LitInt>()?.base10_parse::<u8>()?));
            } else {
                input.parse::<Token![_]>()?;
                frets.push(None);
            }
        }
        frets.reverse();
        let duration_tweak = DurationTweakDsl::try_parse(input);
        ShapeDsl {
            frets,
            duration_tweak,
        }
    }
}

impl ToTokens for ShapeDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ShapeDsl {
            frets,
            duration_tweak,
        } = self;
        //let string_num = Context::fretted().string_num;
        let mut frets_quote: Vec<TokenStream> = vec![];
        let mut fingers_quote: Vec<TokenStream> = vec![];
        for fret in frets {
            frets_quote.push(match fret {
                Some(fret) => quote! { Some(#fret) },
                None => quote! { None },
            });
            fingers_quote.push(quote! { None });
        }
        let duration_quote = Context::duration_quote(duration_tweak);
        let fretted_entry_quote = Context::fretted().fretted_entry_quote();
        let hand_shape_quote = Context::fretted().hand_shape_quote();
        tokens.extend(quote! {
            ProtoEntry::from(#fretted_entry_quote::from(
                (#hand_shape_quote::new([
                    #(#frets_quote),*
                ], [
                    #(#fingers_quote),*
                ]), #duration_quote)
            ))
        });
    }
}
