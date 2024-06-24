use typed_i18n::{BuilderFromRef, BuilderFromValue};

#[derive(PartialEq, Debug)]
pub enum Output<T> {
    Empty,
    Const(&'static str),
    Built(Vec<Element<T>>),
}

#[derive(PartialEq, Debug)]
pub enum Element<T> {
    Const(&'static str),
    String(String),
    T(T),
}

pub struct Tester<T>(Vec<Element<T>>);

impl<T> typed_i18n::Builder for Tester<T> {
    type Output = Output<T>;

    fn empty() -> Self::Output {
        Output::Empty
    }

    fn const_str(i: &'static str) -> Self::Output {
        Output::Const(i)
    }

    fn new() -> Self {
        Tester(Vec::new())
    }

    fn push_const_str(mut self, i: &'static str) -> Self {
        self.0.push(Element::Const(i));
        self
    }

    fn push_str(mut self, i: &str) -> Self {
        self.0.push(Element::String(i.to_string()));
        self
    }

    fn finish(self) -> Self::Output {
        Output::Built(self.0)
    }
}

impl<T> BuilderFromValue<T> for Tester<T> {
    fn push(mut self, i: T) -> Self {
        self.0.push(Element::T(i));
        self
    }
}

impl BuilderFromRef<str> for Tester<String> {
    fn push(mut self, i: &str) -> Self {
        self.0.push(Element::T(i.to_string()));
        self
    }
}
