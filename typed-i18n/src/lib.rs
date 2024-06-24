#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![forbid(missing_docs)]
#![warn(clippy::pedantic)]

//! Convert a language file and an enum into a type safe i18n system.
//!
//! # Basic Usage
//!
//! Yaml language file: (json and lrc is also supported, more below)
//! ```yaml
//! hello_world:
//!   en: Hello World
//!   de: Hallo Welt
//! hello_you:
//!   en: Hello %{name}
//! ```
//!
//! Code:
//! ```rust
//! # use typed_i18n::TypedI18N;
//! #[derive(Copy, Clone, TypedI18N)]
//! #[typed_i18n(filename = "example.yaml")]
//! #[typed_i18n(builder = "mixed_str", prefix = "str_")]
//! enum Language { En, De }
//! ```
//!
//! Generated code:
//! ```rust
//! # enum Language { En, De }
//! impl Language {
//! # } trait LanguageTest {
//!     fn str_hello_world(self) -> &'static str;
//!     fn str_hello_you(self, name: &str) -> String;
//! }
//! ```
//!
//! Usage:
//! ```rust
//! # enum Language { En, De }
//! # impl Language {
//! #    fn str_hello_world(self) -> &'static str {""}
//! #    fn str_hello_you(self, name: &str) -> String {String::new()}
//! # }
//! fn print_hello(language: Language, name: Option<&str>){
//!   if let Some(name) = name {
//!     println!("{}", language.str_hello_you(name));
//!   } else {
//!     println!("{}", language.str_hello_world());
//!   }
//! }
//! ```
//!
//! ## More builders
//!
//! Different generators add different code:
//! ```rust
//! # use typed_i18n::TypedI18N;
//! # #[derive(Copy, Clone, TypedI18N)]
//! # #[typed_i18n(filename = "example.yaml")]
//! #[typed_i18n(builder = "static_str", prefix = "sta_")]
//! #[typed_i18n(builder = "String")]
//! # enum Language { En, De }
//! ```
//!
//! Generated code:
//! ```rust
//! # struct Language;
//! impl Language {
//! # } trait LanguageTest {
//!     fn sta_hello_world(self) -> &'static str;
//!     // The static_str builder skips all parameterized messages
//!     fn hello_world(self) -> String;
//!     fn hello_you(self, name: &str) -> String;
//! }
//! ```
//!
//! ## Other output types
//!
//! Also types other than strings are possible:
//! ```yaml
//! #in addition to the messages above
//! hello_with_icon:
//!   en: Hello %{name}*{icon}
//! ```
//!
//! ```rust
//! # use typed_i18n::{Builder, TypedI18N};
//! # struct HtmlBuilder;
//! # struct Html;
//! # impl Builder for HtmlBuilder {
//! #   type Output = Html;
//! #   fn new() -> Self { todo!() }
//! #   fn push_str(self, i: &str) -> Self { todo!() }
//! #   fn finish(self) -> Self::Output { todo!() }
//! # }
//! # impl typed_i18n::BuilderFromValue<Html> for HtmlBuilder {fn push(self, i: Html) -> Self { todo!() } }
//! # #[derive(Copy, Clone, TypedI18N)]
//! # #[typed_i18n(filename = "example.yaml")]
//! #[typed_i18n(builder = "HtmlBuilder", input = "Html", prefix = "html_")]
//! # enum Language { En, De }
//! ```
//!
//! Generated code:
//! ```rust
//! # use typed_i18n::Builder;
//! # struct HtmlBuilder;
//! # struct Html;
//! # impl Builder for HtmlBuilder {
//! #   type Output = Html;
//! #   fn new() -> Self { todo!() }
//! #   fn push_str(self, i: &str) -> Self { todo!() }
//! #   fn finish(self) -> Self::Output { todo!() }
//! # }
//! # struct Language;
//! impl Language {
//! # } trait LanguageTest {
//!     fn html_hello_world(self) -> <HtmlBuilder as Builder>::Output;
//!     fn html_hello_you(self, name: &str) -> <HtmlBuilder as Builder>::Output;
//!     fn html_hello_with_icon<T1: Into<Html>>(
//!         self,
//!         name: &str,
//!         icon: T1,
//!     ) -> <HtmlBuilder as Builder>::Output;
//! }
//! ```
//! See [examples](https://github.com/alexkazik/typed-i18n/blob/main/examples/src/html.rs) for a `HtmlBuilder` for `yew::Html` implementation.
//!
//! # Input
//!
//! Fields:
//!
//! - `filename`: the path to the translations, relative to the crate root (required).
//! - `separator`: used for combining paths of a tree, default: `_`.
//!
//! Example:
//!
//! `#[typed_i18n(filename = "example.yaml", separator = "_")]`
//!
//! The messages can be supplied in
//! - yaml (as seen in the examples above)
//! - json
//! - lrc
//!
//! json:
//! ```json
//! {"hello_world": {"en": "Hello World"}}
//! ```
//!
//! The yaml and json files may have a `_version: 2` entry, to be compatible
//! with other i18n tools. (Other numbers are an error, omitting it is ok.)
//!
//! The yaml and json files may have a deeper tree. In that case the separator is used to join
//! the segments into a function name.
//!
//! Example:
//! ```yaml
//! hello:
//!   world:
//!     en: Hello World
//!   you:
//!     en: Hello %{name}
//! ```
//! With the separator `_` it will result in the same names as the examples above.
//! This may help you structure the messages.
//!
//! Since the name is a function name a `.` as a separator is not allowed, but you can use `·` (U+00B7).
//!
//! lrc:
//! ```text
//! ; lines starting with `;` or `/` will be skipped
//! #hello_world
//!   en Hello World
//! ```
//!
//! # Output
//!
//! Fields:
//!
//! - `builder`: The name of an item which implements [`Builder`], or a special value.
//! - `prefix`: Prefix for all functions generated by this builder, default: the empty string.
//! - `str_conversion`: How to convert str parameters, default `ref`.
//! - `input`: Type of the input (for typed inputs), default: no input.
//! - `input_conversion`: How to convert the parameter into the input type, default: `into`.
//!
//! ## `builder`
//!
//! Must be either a special value or a type for which [`Builder`] is implemented.
//!
//! All builders without a input type will skip all messages for it.
//!
//! All the builders below are built-in. The `input` must not be set for these. Always available:
//!
//! * `static_str`: all messages without parameters have the return type `&'static str`. All others are skipped.
//! * `mixed_str`: all messages without parameters have the return type `&'static str` all others will have the return type `String`.
//! * `String`
//! * `Cow<'static, str>`
//! * `_`: The functions will be generic over the builder type. This is sometimes not helpful,
//!        e.g. when `.into()` or `.as_ref()` will be called on the result of the function.
//!
//! All except `static_str` and `_` require the feature `alloc` (enabled by default) and that `String` or `Cow` is in scope.
//!
//! To learn about custom type builder see the example above and in the [examples directory](https://github.com/alexkazik/typed-i18n/blob/main/examples).
//!
//! ## `prefix`
//!
//! All functions of this `builder` will be composed of the prefix and the message name.
//!
//! Using identical prefixes or overlapping names will result in functions with identical names and
//! thus result in a compile error. This will not be checked by the derive macro.
//!
//! ## `str_conversion`
//!
//! The conversion for all string (normal) `%{param}` parameters:
//!
//! * `ref` (default): `fn str_hello_you(self, name: &str) -> String`.
//! * `as_ref`: `fn str_hello_you<S1: AsRef<str>>(self, name: S1) -> String`.
//!
//! ## `input`
//!
//! Type of the input for typed `*{param}` parameters.
//!
//! All builders without `input` will silently skip all messages with typed parameters.
//!
//! With `_` a generic function over the input type is created. May lead to the same problems as
//! a generic builder type.
//!
//! ## `input_conversion`
//!
//! How to convert the types inputs:
//!
//! * `value`: The input type as parameter:
//!             `fn html_hello_with_icon(self, name: &str, icon: Input) -> <HtmlBuilder as BuilderFromValue>::Output`
//! * `into` (default): Something that can be converted into the input type:
//!             `fn html_hello_with_icon<T1: Into<Input>>(self, name: &str, icon: T1) -> <HtmlBuilder as BuilderFromValue>::Output`
//! * `ref`: A reference to the input type:
//!             `fn html_hello_with_icon(self, name: &str, icon: &Input) -> <HtmlBuilder as BuilderFromRef>::Output`
//! * `as_ref`: Something that can be converted into a reference to the input type:
//!             `fn html_hello_with_icon<T1: AsRef<Input>>(self, name: &str, icon: T1) -> <HtmlBuilder as BuilderFromRef>::Output`
//!
//! For `value` and `into` to work the builder must also implement [`BuilderFromValue`].
//!
//! For `ref` and `as_ref` to work the builder must also implement [`BuilderFromRef`].
//!
//! # Language
//!
//! The enum values can be annotated with:
//!
//! * `name`: The name of the language in the messages file, defaults to the name of the value in `snake_case`.
//! * `fallback`: A space and/or comma separated list of language names which defines which language
//!   should be used when a message is missing. Default: all languages in listing order (not necessary in numerical order).
//!
//! Example:
//!
//! ```rust
//! # use typed_i18n::TypedI18N;
//! #[derive(Copy, Clone, TypedI18N)]
//! #[typed_i18n(filename = "example.yaml")]
//! #[typed_i18n(builder = "mixed_str", prefix = "str_")]
//! enum Language {
//!   De,
//!   #[typed_i18n(name = "en")]
//!   English,
//!   #[typed_i18n(fallback = "en, de")]
//!   EnAu,
//! }
//! ```
//!
//! # Features
//!
//! - `alloc`, enabled by default: Provide Builder implementations for `String` and `Cow<'static, str>`, also support `mixed_str`.
//!
//! The library is always `no_std`.

#[cfg(feature = "alloc")]
mod alloc;

pub use typed_i18n_derive::TypedI18N;

/// Trait to create localized strings (from constant messages and string parameters).
pub trait Builder: Sized {
    /// The output type.
    type Output;

    /// Create a new empty result.
    #[inline]
    #[must_use]
    fn empty() -> Self::Output {
        Self::const_str("")
    }

    /// Create a new result from a const string.
    #[inline]
    #[must_use]
    fn const_str(i: &'static str) -> Self::Output {
        Self::new().push_const_str(i).finish()
    }

    /// Create a new builder.
    #[must_use]
    fn new() -> Self;

    /// Add a const string to the builder.
    #[inline]
    #[must_use]
    fn push_const_str(self, i: &'static str) -> Self {
        self.push_str(i)
    }

    /// Add a string to the builder.
    #[must_use]
    fn push_str(self, i: &str) -> Self;

    /// Convert the builder into the output.
    #[must_use]
    fn finish(self) -> Self::Output;
}

/// Trait to create localized strings from a reference to a value.
pub trait BuilderFromRef<Input: ?Sized>: Builder {
    /// Add a typed parameter by reference to the builder.
    #[must_use]
    fn push(self, i: &Input) -> Self;
}

/// Trait to create localized strings from a value.
pub trait BuilderFromValue<Input>: Builder {
    /// Add a typed parameter by value to the builder.
    #[must_use]
    fn push(self, i: Input) -> Self;
}
