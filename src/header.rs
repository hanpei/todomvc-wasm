#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::{TodoItem, Todos};

#[derive(Debug, Props, PartialEq)]
pub struct TodoInputProps<'a> {
    todos: &'a UseState<Todos>,
}

pub fn TodoInput<'a>(cx: Scope<'a, TodoInputProps>) -> Element<'a> {
    let draft = use_state(&cx, || "".to_string());
    let &TodoInputProps { todos } = cx.props;
    let todo_id = cx.props.todos.next_id;

    rsx!(
        cx,
        header {
            class: "header",
            h1{"Todos"},
            input {
                class: "new-todo",
                placeholder: "What needs to be done?",
                autofocus: "true",
                value: "{draft}",
                oninput: move |evt| {
                    draft.set(evt.value.clone());

                },
                onkeydown: move |evt| {
                    if evt.key == "Enter" && !draft.is_empty() {
                        let mut todos = todos.make_mut();
                        todos.insert(
                            todo_id,
                            TodoItem::new(draft.get().clone()),
                        );
                        todos.save();

                        todos.next_id += 1;
                        draft.set("".to_string());
                    }
                }
            }
        }
    )
}
