use crate::attribute::builder::{BuilderVariant, InputConversion, InputVariant, StrConversion};
use crate::attribute::{Attributes, Builder, Global};
use crate::diagnostic::Diagnostic;
use crate::languages::{Language, Languages};
use crate::messages::message::Message;
use crate::messages::message_line::MessageLine;
use crate::messages::messages::Messages;
use crate::messages::param_type::ParamType;
use crate::messages::piece::Piece;
use convert_case::{Case, Casing};
use indexmap::IndexMap;
use proc_macro2::Ident;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::borrow::Cow;
use syn::{LitInt, Type, Visibility};

impl Attributes {
    pub fn generate<D: Diagnostic>(
        &self,
        diagnostic: &mut D,
        vis: &Visibility,
        enum_ident: &Ident,
        relative_path: &str,
        languages: &Languages,
        messages: &Messages,
    ) -> TokenStream {
        diagnostic.should_abort_if_dirty();

        let mut inner = TokenStream::new();

        inner.extend(quote!(
            const _DEPENDENCY: &'static str = include_str!(#relative_path);
        ));

        for builder in &self.builders {
            builder.generate(diagnostic, vis, enum_ident, languages, messages, &mut inner);
        }

        let global = self.parameters.global.map_or(TokenStream::new(), |g| {
            g.generate(vis, enum_ident, languages)
        });

        quote!(
            impl #enum_ident where #enum_ident : ::std::marker::Copy {
                #inner
            }
            #global
        )
    }
}

impl Builder {
    fn generate<D: Diagnostic>(
        &self,
        diagnostic: &mut D,
        vis: &Visibility,
        enum_ident: &Ident,
        languages: &Languages,
        messages: &Messages,
        output: &mut TokenStream,
    ) {
        let prefix = self.prefix.as_deref().unwrap_or("");

        if (self.builder_variant == BuilderVariant::StaticStr
            || self.builder_variant == BuilderVariant::MixedStr)
            && self.input_variant != InputVariant::None
        {
            diagnostic.emit_error(self.span, "special builder can't have an input");

            return;
        }
        for (
            k,
            Message {
                params,
                message_lines,
                ..
            },
        ) in messages
        {
            if self.builder_variant == BuilderVariant::StaticStr && !params.is_empty() {
                // non-static result
                continue;
            }
            if self.input_variant == InputVariant::None
                && message_lines.iter().any(|(_, m)| {
                    m.borrow_pieces()
                        .iter()
                        .any(|mm| matches!(mm, Piece::Param(_, ParamType::Typed)))
                })
            {
                // non-input builder with input message
                continue;
            }
            let fn_ident = Ident::new(&format!("{prefix}{k}"), self.span);
            output.extend(quote!(
                #vis fn #fn_ident
            ));
            let mut generics = Vec::new();
            let mut args = TokenStream::new();
            args.extend(quote!(self,));
            for (p_name, p_type) in params {
                let p_name_ident = Ident::new(p_name, self.span);
                if *p_type == ParamType::Str {
                    if self.str_conversion == StrConversion::Ref {
                        args.extend(quote!(#p_name_ident : &str,));
                    } else {
                        let input =
                            Ident::new(&format!("S{}", generics.len() + 1), Span::call_site());
                        args.extend(quote!(#p_name_ident : #input,));
                        generics.push((*p_type, input));
                    }
                } else if self.input_conversion == InputConversion::Ref {
                    let input_ident = &self.input_ident;
                    args.extend(quote!(#p_name_ident : &#input_ident,));
                } else if self.input_conversion == InputConversion::Value {
                    let input_ident = &self.input_ident;
                    args.extend(quote!(#p_name_ident : #input_ident,));
                } else {
                    let input = Ident::new(&format!("T{}", generics.len() + 1), Span::call_site());
                    args.extend(quote!(#p_name_ident : #input,));
                    generics.push((*p_type, input));
                }
            }
            if !generics.is_empty() || self.builder_variant == BuilderVariant::Generic {
                let input_ident = &self.input_ident;
                output.extend(quote!(<));
                if self.builder_variant == BuilderVariant::Generic {
                    output.extend(quote!(T,));
                }
                if self.input_variant == InputVariant::Generic
                    && generics.iter().any(|(t, _)| *t == ParamType::Typed)
                {
                    output.extend(quote!(#input_ident,));
                }
                for (_, g) in &generics {
                    output.extend(quote!(#g,));
                }
                output.extend(quote!(>));
            }
            output.extend(quote!((#args)));
            let return_static_str = (self.builder_variant == BuilderVariant::StaticStr
                || self.builder_variant == BuilderVariant::MixedStr)
                && params.is_empty();
            let builder_type = &self.builder_type;
            if return_static_str {
                output.extend(quote!( -> &'static str ));
            } else if self.builder_variant == BuilderVariant::MixedStr {
                output.extend(quote!( -> String ));
            } else if self.builder_variant == BuilderVariant::Generic {
                output.extend(quote!( -> T::Output ));
            } else {
                output.extend(quote!( -> <#builder_type as ::typed_i18n::Builder>::Output ));
            }
            if !generics.is_empty() {
                let input_ident = &self.input_ident;
                output.extend(quote!(where));
                if generics.iter().any(|(t, _)| *t == ParamType::Typed) {
                    if self.input_conversion == InputConversion::AsRef {
                        output.extend(
                            quote!(#builder_type : ::typed_i18n::BuilderFromRef<#input_ident>,),
                        );
                    } else {
                        output.extend(
                            quote!(#builder_type : ::typed_i18n::BuilderFromValue<#input_ident>,),
                        );
                    }
                }
                for (t, g) in &generics {
                    match *t {
                        ParamType::Str => output.extend(quote!(#g : ::core::convert::AsRef<str>,)),
                        ParamType::Typed => match self.input_conversion {
                            InputConversion::Into => {
                                output.extend(quote!(#g : ::core::convert::Into<#input_ident>,));
                            }
                            InputConversion::AsRef => {
                                output.extend(quote!(#g : ::core::convert::AsRef<#input_ident>,));
                            }
                            InputConversion::Value | InputConversion::Ref => {}
                        },
                    }
                }
            } else if self.builder_variant == BuilderVariant::Generic {
                output.extend(quote!(where #builder_type : ::typed_i18n::Builder,));
            }
            let body = languages.generate(
                builder_type,
                enum_ident,
                return_static_str,
                self.str_conversion,
                self.input_conversion,
                message_lines,
            );
            output.extend(quote!({#body}));
        }
    }
}

impl Languages {
    fn generate(
        &self,
        builder_type: &Type,
        enum_ident: &Ident,
        return_static_str: bool,
        str_conversion: StrConversion,
        input_conversion: InputConversion,
        m: &IndexMap<Cow<'_, str>, MessageLine<'_>>,
    ) -> TokenStream {
        if m.len() == 1 {
            return self.iter().next().expect("no language!").generate(
                builder_type,
                return_static_str,
                str_conversion,
                input_conversion,
                m,
            );
        }

        let mut body = TokenStream::new();
        for l in self {
            let gl = l.generate(
                builder_type,
                return_static_str,
                str_conversion,
                input_conversion,
                m,
            );
            let lang_ident = &l.ident;
            body.extend(quote!(#enum_ident :: #lang_ident => {#gl},));
        }
        quote!(match self { #body })
    }
}

impl Language {
    fn generate(
        &self,
        builder_type: &Type,
        return_static_str: bool,
        str_conversion: StrConversion,
        input_conversion: InputConversion,
        m: &IndexMap<Cow<'_, str>, MessageLine<'_>>,
    ) -> TokenStream {
        let m = self
            .fallback
            .iter()
            .find_map(|l| m.get(l.as_str()))
            .expect("couldn't find a value")
            .borrow_pieces()
            .as_slice();

        if return_static_str {
            return match m.first() {
                None => quote!(""),
                Some(Piece::Text(t)) => quote!(#t),
                _ => panic!("invalid return_static_str"),
            };
        }
        if let &[Piece::Text(t)] = &m {
            return quote!(<#builder_type as ::typed_i18n::Builder>::const_str(#t));
        }
        if m.is_empty() {
            return quote!(<#builder_type as ::typed_i18n::Builder>::empty());
        }
        let mut body = quote!(<#builder_type as ::typed_i18n::Builder>::new());
        for p in m {
            match p {
                Piece::Text(t) => {
                    body = quote!(::typed_i18n::Builder::push_const_str(#body, #t));
                }
                Piece::Param(p, ParamType::Str) => {
                    let p = Ident::new(p, Span::call_site());
                    body = match str_conversion {
                        StrConversion::Ref => quote!(::typed_i18n::Builder::push_str(#body, #p)),
                        StrConversion::AsRef => {
                            quote!(::typed_i18n::Builder::push_str(#body, #p.as_ref()))
                        }
                    };
                }
                Piece::Param(p, ParamType::Typed) => {
                    let p = Ident::new(p, Span::call_site());
                    body = match input_conversion {
                        InputConversion::Value => {
                            quote!(::typed_i18n::BuilderFromValue::push(#body, #p))
                        }
                        InputConversion::Into => {
                            quote!(::typed_i18n::BuilderFromValue::push(#body, #p.into()))
                        }
                        InputConversion::Ref => {
                            quote!(::typed_i18n::BuilderFromRef::push(#body, #p))
                        }
                        InputConversion::AsRef => {
                            quote!(::typed_i18n::BuilderFromRef::push(#body, #p.as_ref()))
                        }
                    };
                }
            }
        }
        quote!(::typed_i18n::Builder::finish(#body))
    }
}

impl Global {
    #[allow(clippy::missing_panics_doc)]
    fn generate(self, vis: &Visibility, enum_ident: &Ident, languages: &Languages) -> TokenStream {
        match self {
            Global::Atomic => {
                let static_name = Ident::new(
                    &format!(
                        "STATIC_{}",
                        enum_ident.to_string().to_case(Case::UpperSnake)
                    ),
                    enum_ident.span(),
                );

                let num_languages = languages.iter().count();

                let atomic_type = if num_languages <= 256 {
                    Ident::new("AtomicU8", Span::call_site())
                } else {
                    Ident::new("AtomicUsize", Span::call_site())
                };

                let mut inner_match = TokenStream::new();
                let mut inner_table = TokenStream::new();
                for (i, l) in languages.iter().enumerate() {
                    let language_ident = &l.ident;
                    let i = LitInt::new(&i.to_string(), Span::call_site());
                    inner_match.extend(quote! {Self::#language_ident => #i,});
                    inner_table.extend(quote! {Self::#language_ident,});
                }

                let default_language = languages
                    .iter()
                    .enumerate()
                    .find(|(_, l)| l.default)
                    .unwrap()
                    .0;
                let default_language =
                    LitInt::new(&default_language.to_string(), Span::call_site());

                quote! {
                    static #static_name: ::core::sync::atomic::#atomic_type = ::core::sync::atomic::#atomic_type::new(#default_language);

                    impl #enum_ident {
                        const FROM_INDEX : &'static [Self; #num_languages] = &[#inner_table];

                        #vis fn set_global(self) {
                            #static_name.store(match self { #inner_match }, ::core::sync::atomic::Ordering::Relaxed);
                        }

                        #vis fn global() -> Self {
                            Self::FROM_INDEX[#static_name.load(::core::sync::atomic::Ordering::Relaxed) as usize]
                        }
                    }
                }
            }
        }
    }
}
