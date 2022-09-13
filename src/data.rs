#![allow(non_snake_case)]
use druid::{im::Vector, Data, Lens, EventCtx, Env, ArcStr, KeyOrValue};
use std::fs::File;
use std::io::{BufReader, Error};
use druid::widget::TextBox;
use druid::text::{RichText, Attribute};

#[derive(Clone, Data, Lens)]
pub struct AppState {
    new_todo: String,
    todos: Vector<TodoItem>,
    rich_text: RichText
}

impl AppState {
    pub fn new(todos: Vec<TodoItem>) -> Self {
        Self {
            new_todo: "".into(),
            todos: Vector::from(todos),
            rich_text: RichText::new(ArcStr::from("Oidocrop")).with_attribute(0..20,Attribute::FontSize(KeyOrValue::Concrete(50.0)))
        }
    }

    pub(crate)  fn add_todo(&mut self) {
        self.todos.push_front(TodoItem::new(&self.new_todo));
        self.new_todo = "".into();
        self.save_to_json().unwrap();
    }

    pub fn click_add(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.add_todo();
    }

    pub fn save_to_json(&self) -> Result<(), Error> {
        let todo_vec: Vec<TodoItem> = self.todos.iter().map(|item| item.to_owned()).collect();
        let serialized = serde_json::to_string_pretty(&todo_vec)?;
        std::fs::write("todos.json", serialized)?;
        Ok(())
    }

    pub fn load_from_json() -> Self {
        let file = File::open("todos.json");

        match file {
            Ok(file) => {
                let reader = BufReader::new(file);
                let todos: Vec<TodoItem> = serde_json::from_reader(reader).unwrap_or(vec![]);
                Self {
                    todos: Vector::from(todos),
                    new_todo: String::new(),
                    rich_text: RichText::new(ArcStr::from("Oidocrop")),
                }
            }
            Err(_) => Self {
                todos: Vector::new(),
                new_todo: String::new(),
                rich_text: RichText::new(ArcStr::from("Oidocrop")),
            },
        }
    }
}

#[derive(Clone, Data, Lens, serde::Serialize, serde::Deserialize)]
pub struct TodoItem {
    pub(crate) done: bool,
    pub text: String,
}

impl TodoItem {
    pub fn new(text: &str) -> Self {
        Self {
            done: false,
            text: text.into(),
        }
    }
}

