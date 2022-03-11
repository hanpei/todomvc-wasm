#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::*;

pub fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        section{ class: "todoapp",

            style { [include_str!("../src/style.css")] },
            TodosContainer()
        }
    ))
}
