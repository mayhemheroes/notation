use std::sync::RwLock;

use crate::core::duration::DurationTweakDsl;
use crate::core::octave::OctaveTweakDsl;
use fehler::{throw, throws};
use notation_proto::prelude::{
    Duration, Key, Note, Octave, Scale, Syllable, SyllableNote, GUITAR_STRING_NUM,
};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, Parse, ParseStream};
use syn::{Ident, LitInt, Token};

lazy_static! {
    static ref CONTEXT: RwLock<Context> = RwLock::new(Context::default());
}

#[derive(Copy, Clone, Debug)]
pub struct FrettedContext {
    pub string_num: usize,
}
impl Default for FrettedContext {
    fn default() -> Self {
        Self {
            string_num: GUITAR_STRING_NUM,
        }
    }
}
impl FrettedContext {
    pub fn fretted_entry_quote(&self) -> TokenStream {
        if self.string_num == 6 {
            quote! { FrettedEntry6 }
        } else if self.string_num == 4 {
            quote! { FrettedEntry4 }
        } else {
            panic!("Unsupported string_num: {}", self.string_num);
        }
    }
    pub fn fretboard_quote(&self) -> TokenStream {
        if self.string_num == 6 {
            quote! { Fretboard6 }
        } else if self.string_num == 4 {
            quote! { Fretboard4 }
        } else {
            panic!("Unsupported string_num: {}", self.string_num);
        }
    }
    pub fn hand_shape_quote(&self) -> TokenStream {
        if self.string_num == 6 {
            quote! { HandShape6 }
        } else if self.string_num == 4 {
            quote! { HandShape4 }
        } else {
            panic!("Unsupported string_num: {}", self.string_num);
        }
    }
}

#[derive(Debug)]
pub struct Context {
    pub key: Key,
    pub scale: Scale,
    pub duration: Duration,
    pub octave: Octave,
    pub fretted: FrettedContext,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            key: Key::default(),
            scale: Scale::default(),
            duration: Duration::default(),
            octave: Octave::default(),
            fretted: FrettedContext::default(),
        }
    }
}

impl Context {
    pub fn key() -> Key {
        CONTEXT.read().unwrap().key
    }
    pub fn scale() -> Scale {
        CONTEXT.read().unwrap().scale
    }
    pub fn duration() -> Duration {
        CONTEXT.read().unwrap().duration
    }
    pub fn base_octave() -> Octave {
        CONTEXT.read().unwrap().octave
    }
    pub fn fretted() -> FrettedContext {
        CONTEXT.read().unwrap().fretted
    }
}

impl Context {
    pub fn duration_quote(tweak: &Option<DurationTweakDsl>) -> TokenStream {
        let base = Self::duration();
        let duration = tweak.as_ref().map(|t| t.tweak(&base)).unwrap_or(base);
        let ident = duration.to_ident();
        quote! {
            Duration::from_ident(#ident)
        }
    }
    pub fn octave(tweak: &Option<OctaveTweakDsl>) -> Octave {
        let base = Self::base_octave();
        tweak.as_ref().map(|t| t.tweak(&base)).unwrap_or(base)
    }
    pub fn octave_quote(tweak: &Option<OctaveTweakDsl>) -> TokenStream {
        let ident = Self::octave(tweak).to_ident();
        quote! {
            Octave::from_ident(#ident)
        }
    }
    pub fn calc_note(tweak: &Option<OctaveTweakDsl>, syllable: &Syllable) -> Note {
        let octave = Self::octave(tweak);
        let key = Self::key();
        let scale = Self::scale();
        scale.calc_note(&key, &SyllableNote::new(octave, *syllable))
    }
    pub fn calc_note_quote(tweak: &Option<OctaveTweakDsl>, syllable: &Syllable) -> TokenStream {
        let note = Self::calc_note(tweak, syllable);
        let octave_ident = note.octave.to_ident();
        let pitch_text = note.pitch.to_text();
        quote! {
            Note::new(Octave::from_ident(#octave_ident), Pitch::from_text(#pitch_text))
        }
    }
}

pub enum ContextDsl {
    Key(Ident),
    Scale(Ident),
    Duration(Ident),
    Octave(Ident),
    StringNum(usize),
}

impl Parse for ContextDsl {
    #[throws(Error)]
    fn parse(input: ParseStream) -> Self {
        input.parse::<Token![$]>()?;
        match input.parse::<Ident>()?.to_string().as_str() {
            "key" => {
                input.parse::<Token![=]>()?;
                Self::Key(input.parse()?)
            }
            "scale" => {
                input.parse::<Token![=]>()?;
                Self::Scale(input.parse()?)
            }
            "duration" => {
                input.parse::<Token![=]>()?;
                Self::Duration(input.parse()?)
            }
            "octave" => {
                input.parse::<Token![=]>()?;
                Self::Octave(input.parse()?)
            }
            "string_num" => {
                input.parse::<Token![=]>()?;
                let string_num = input.parse::<LitInt>()?.base10_parse::<usize>()?;
                Self::StringNum(string_num)
            }
            _ => throw!(Error::new(input.span(), "Invalid Context")),
        }
    }
}

impl ContextDsl {
    pub fn peek(input: ParseStream) -> bool {
        input.peek(Token![$])
    }
}

impl ToTokens for ContextDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Self::Key(x) => {
                CONTEXT.write().unwrap().key = Key::from_ident(x.to_string().as_str());
                let comment = format!("{}", Context::key());
                quote! {
                    ProtoEntry::from(("dsl::context::key", #comment))
                }
            }
            Self::Scale(x) => {
                CONTEXT.write().unwrap().scale = Scale::from_ident(x.to_string().as_str());
                let comment = format!("{}", Context::scale());
                quote! {
                    ProtoEntry::from(("dsl::context::scale", #comment))
                }
            }
            Self::Duration(x) => {
                CONTEXT.write().unwrap().duration = Duration::from_ident(x.to_string().as_str());
                let comment = format!("{}", Context::duration());
                quote! {
                    ProtoEntry::from(("dsl::context::duration", #comment))
                }
            }
            Self::Octave(x) => {
                CONTEXT.write().unwrap().octave = Octave::from_ident(x.to_string().as_str());
                let comment = format!("{}", Context::base_octave());
                quote! {
                    ProtoEntry::from(("dsl::context::octave", #comment))
                }
            }
            Self::StringNum(x) => {
                CONTEXT.write().unwrap().fretted.string_num = *x;
                let comment = format!("{}", Context::fretted().string_num);
                quote! {
                    ProtoEntry::from(("dsl::context::string_num", #comment))
                }
            }
        });
    }
}
