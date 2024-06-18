use examples::HtmlBuilder;
use typed_i18n::TypedI18N;
use yew::virtual_dom::{VNode, VText};
use yew::{AttrValue, Html};

#[derive(Copy, Clone, TypedI18N)]
#[typed_i18n(filename = "demo.yaml", separator = "·")]
#[typed_i18n(
    builder = "HtmlBuilder",
    str_conversion = "as_ref",
    input = "Html",
    input_conversion = "ref"
)]
enum Language {
    En,
    De,
}

fn main() {
    let icon = VNode::VText(VText {
        text: AttrValue::Static("icon!"),
    });
    let name: &String = &"name".to_string();
    let _en = Language::En.hello·you(name, &icon);
    let _de = Language::De.hello·you(name, &icon);
}
