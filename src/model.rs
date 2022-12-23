use serde::{Deserialize, Serialize};
use super::utils;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoEntry {
    pub title: String,
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoList {
    entries: Vec<TodoEntry>
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList { entries: Vec::new() }
    }

    pub fn load(path: &str) -> TodoList {
        match utils::load::<TodoList>(&path) {
            Ok(maybe_data) => maybe_data.unwrap_or(TodoList::new()),
            Err(error) => panic!("Error loading data: {}", error),
        }
    }

    pub fn save(self: &Self, path: &str) -> Result<(), String> {
        utils::save::<TodoList>(&path, &self)
    }

    pub fn entries(self: &Self) -> &Vec<TodoEntry> {
        &self.entries
    }

    pub fn add_entry(self: & mut Self, entry: TodoEntry) -> () {
        self.entries.push(entry);
    }

    pub fn remove_entry(self: &mut Self, index: usize) -> Result<(), String> {
        if self.entries.len() > index {
            self.entries.remove(index);
            Ok(())
        } else {
            Err(format!("No such entry: {}", index))
        }
    }
}