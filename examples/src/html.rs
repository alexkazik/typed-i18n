use std::mem;
use std::rc::Rc;
use typed_i18n::{Builder, BuilderFromRef, BuilderFromValue};
use yew::virtual_dom::{VList, VNode, VText};
use yew::{AttrValue, Html};

pub struct HtmlBuilder {
    h: Vec<Html>,
    v: Value,
}

enum Value {
    None,
    Const(&'static str),
    String(String),
}

impl HtmlBuilder {
    fn clean(&mut self) {
        match mem::replace(&mut self.v, Value::None) {
            Value::None => {}
            Value::Const(c) => {
                self.h.push(VNode::VText(VText {
                    text: AttrValue::Static(c),
                }));
            }
            Value::String(s) => {
                self.h.push(VNode::VText(VText {
                    text: AttrValue::Rc(Rc::from(s.as_str())),
                }));
            }
        }
    }
}

impl Builder for HtmlBuilder {
    type Output = Html;

    #[inline]
    fn empty() -> Self::Output {
        VNode::VList(VList::new())
    }

    #[inline]
    fn const_str(i: &'static str) -> Self::Output {
        VNode::VText(VText {
            text: AttrValue::Static(i),
        })
    }

    #[inline]
    fn new() -> Self {
        Self {
            h: Vec::new(),
            v: Value::None,
        }
    }

    #[inline]
    fn push_const_str(mut self, i: &'static str) -> Self {
        match &mut self.v {
            Value::None => {
                self.v = Value::Const(i);
            }
            Value::Const(c) => {
                let mut s = (**c).to_string();
                String::push_str(&mut s, i);
                self.v = Value::String(s);
            }
            Value::String(s) => {
                String::push_str(s, i);
            }
        }
        self
    }

    #[inline]
    fn push_str(mut self, i: &str) -> Self {
        match &mut self.v {
            Value::None => {
                self.v = Value::String(i.to_string());
            }
            Value::Const(c) => {
                let mut s = (**c).to_string();
                String::push_str(&mut s, i);
                self.v = Value::String(s);
            }
            Value::String(s) => {
                String::push_str(s, i);
            }
        }
        self
    }

    #[inline]
    fn finish(mut self) -> Self::Output {
        self.clean();
        if self.h.len() == 1 {
            self.h.into_iter().next().unwrap()
        } else {
            VNode::VList(VList::with_children(self.h, None))
        }
    }
}

// This probably should not be used as it's better to use clone directly
// to make clear that it's used. It's here for demonstration purposes.
// (And to simplify the examples.)
impl BuilderFromRef<Html> for HtmlBuilder {
    #[inline]
    fn push(mut self, i: &Html) -> Self {
        self.clean();
        self.h.push(i.clone());
        self
    }
}

impl BuilderFromValue<Html> for HtmlBuilder {
    #[inline]
    fn push(mut self, i: Html) -> Self {
        self.clean();
        self.h.push(i);
        self
    }
}
