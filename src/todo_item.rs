#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::info;

use crate::Todos;

#[derive(Debug, Props, PartialEq)]
pub struct TodoItemProps<'a> {
    id: usize,
    title: String,
    complete: bool,
    todos: &'a UseState<Todos>,
    editing: &'a UseState<usize>,
}

pub fn Item<'a>(cx: Scope<'a, TodoItemProps>) -> Element<'a> {
    let TodoItemProps {
        id,
        title,
        complete,
        todos,
        editing,
    } = cx.props;

    let completed = if *complete { "completed" } else { "" };

    let is_editing = if editing.get() == id { "editing" } else { "" };

    rsx!(cx,
        li {
            class: "{completed} {is_editing}",
            key: "{id}",

            div {
                class: "view",
                input {
                    class:"toggle",
                    r#type:"checkbox",
                    checked: "{complete}",
                    onclick: move |_| {
                        let mut todos = todos.make_mut();
                        todos.get_mut(id).map(|todo| {
                            todo.complete = !todo.complete
                        });
                        todos.save();
                    }
                }
                label {
                    onclick: move |_| {
                        info!("double click");
                        editing.set(*id);
                    },
                    "{title}",
                }
                button {
                    class: "destroy",
                    onclick: move |_| {
                        let mut todos = todos.make_mut();
                        todos.remove(id);
                        todos.save();
                    }
                }
            },

            (*editing.get() > 0).then(||{
                rsx!(
                    input {
                        class: "edit",
                        autofocus: "true",
                        value: "{title}",
                        oninput: move |e| {
                            let mut todos = todos.make_mut();
                            todos.get_mut(id).map(|todo| {
                                todo.title = e.value.parse().unwrap();
                            });
                            todos.save();

                        },
                        onblur: move |_| {
                            editing.set(0);
                            info!("onblur")
                        },
                        onkeydown: move |evt| {
                            match evt.key.as_str() {
                                "Enter" | "Escape" => editing.set(0),
                                _ =>()

                            }
                        },
                    }
                )
            }),
        }
    )
}
