use dioxus::prelude::*;
use crate::prelude::*;

pub fn Button<'a>(cx: Scope<'a, ItemProps<'a>>) -> Element {
    cx.render(rsx!(
        button {
            class: class!(button::btn) + cx.props.class,
            &cx.props.children
        }
    ))
}