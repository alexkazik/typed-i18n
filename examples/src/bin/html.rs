use examples::HtmlBuilder;
use typed_i18n::TypedI18N;
use yew::virtual_dom::{ApplyAttributeAs, Attributes, VNode, VTag};
use yew::Html;

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
    let icon = VNode::VTag(Box::new({
        let mut icon = VTag::new("img");
        icon.attributes = Attributes::Static(&[("href", "icon.png", ApplyAttributeAs::Attribute)]);
        icon
    }));
    let name: &dyn AsRef<str> = &"name".to_string();
    let _en = Language::En.hello·you(name, &icon);
    let _de = Language::De.hello·you(name, &icon);
}
