mod app;
mod footer;
mod header;
mod storage;
mod todo_item;
mod todos;

use std::{
    collections::BTreeMap,
    fmt::Display,
    ops::{Deref, DerefMut},
};

pub use app::*;
pub use footer::*;
pub use header::*;
use serde::{Deserialize, Serialize};
pub use storage::*;
pub use todo_item::*;
pub use todos::*;

use web_sys::window;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Todos {
    pub store: BTreeMap<usize, TodoItem>,
    pub next_id: usize,
}

impl Todos {
    pub fn save(&mut self) {
        set_storage(self);
    }
}

impl Deref for Todos {
    type Target = BTreeMap<usize, TodoItem>;

    fn deref(&self) -> &Self::Target {
        &self.store
    }
}
impl DerefMut for Todos {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.store
    }
}

impl Default for Todos {
    fn default() -> Self {
        Todos {
            store: BTreeMap::new(),
            next_id: 1,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    pub title: String,
    pub complete: bool,
}

impl TodoItem {
    pub fn new(title: String) -> Self {
        Self {
            title,
            complete: false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl Default for Filter {
    fn default() -> Self {
        let hash = window().unwrap().location().hash().unwrap_or_default();
        match hash.as_str() {
            "#/all" => Filter::All,
            "#/active" => Filter::Active,
            "#/completed" => Filter::Completed,
            _ => Filter::All,
        }
    }
}

impl From<&str> for Filter {
    fn from(s: &str) -> Self {
        match s {
            "All" => Self::All,
            "Active" => Self::Active,
            "Completed" => Self::Completed,
            _ => Self::default(),
        }
    }
}

impl Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "All"),
            Self::Active => write!(f, "Active"),
            Self::Completed => write!(f, "Completed"),
        }
    }
}
