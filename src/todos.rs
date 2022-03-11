#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::{get_storage, Filter, Footer, Item, TodoInput};

pub fn TodosContainer(cx: Scope) -> Element {
    let todos = use_state(&cx, || get_storage());
    let filter = use_state(&cx, Filter::default);
    let editing = use_state(&cx, || 0_usize);

    let filtered_ids: Vec<usize> = todos
        .get()
        .iter()
        .filter(|(_, item)| match filter.get() {
            Filter::All => true,
            Filter::Active => !item.complete,
            Filter::Completed => item.complete,
        })
        .map(|(i, _)| (*i))
        .collect();

    let item_left = todos
        .get()
        .iter()
        .filter(|(_, item)| !item.complete)
        .count();

    rsx!(
        cx,
        TodoInput {
            todos: todos
        },
        section {
            class: "main",
            prevent_default: "true",
            ul {
                class: "todo-list",

                filtered_ids.iter().map(|id| {
                    let item = todos.get().get(id).unwrap();
                    rsx!(
                        Item {
                            id:*id,
                            title:item.title.clone(),
                            complete: item.complete,
                            todos:todos,
                            editing:editing,
                        }
                    )
                })
            }
        }
        Footer{filter: filter, item_left: item_left}
    )
}
