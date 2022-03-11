use serde_json;
use web_sys::{window, Storage};

use crate::Todos;

const STORAGE_KEY: &str = "todos";

fn storage() -> Storage {
    window()
        .unwrap()
        .local_storage()
        .unwrap()
        .expect("window.local_storage")
}

pub fn get_storage() -> Todos {
    let storage = storage();
    let s = storage.get(STORAGE_KEY).unwrap_or_default();
    match s {
        Some(s) => serde_json::from_str(&s).unwrap(),
        None => Todos::default(),
    }
}

pub fn set_storage(todos: &Todos) {
    let storage = storage();
    let s = serde_json::to_string(todos).unwrap();
    storage.set(STORAGE_KEY, &s).unwrap();
}
