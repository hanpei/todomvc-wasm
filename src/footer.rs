#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::Filter;

#[derive(Debug, Props, PartialEq)]
pub struct FooterProps<'a> {
    filter: &'a UseState<Filter>,
    item_left: usize,
}

pub fn Footer<'a>(cx: Scope<'a, FooterProps>) -> Element<'a> {
    let item_left = cx.props.item_left;

    let filter = cx.props.filter;
    let selected = use_state(&cx, || Filter::default().to_string());

    cx.render(rsx!(
        footer {
            class: "footer",
            span {
                class:"todo-count",
                strong { "{item_left} " }
                "item left"
            }

            ul {
                class: "filters",
                ["All", "Active", "Completed"].map(|current| {

                    let active=  if selected.get() == current {
                        "selected"
                    } else {
                        ""
                    };
                    let href = current.to_string().to_lowercase();
                    rsx!(
                        li {
                            a {
                                class:"{active}",
                                onclick: move |_| {
                                    selected.set(current.into());
                                    filter.set(current.into());
                                },
                                href: "#/{href}",
                                "{current}"
                            }
                        }
                    )
                })

            }
        }
    ))
}
