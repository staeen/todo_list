use serde::{Serialize, Deserialize};
use std::fs::OpenOptions;
use std::io::{Read, Write};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    id: u32,
    description: String,
    completed: bool,
}

const FILE_PATH: &str = "tasks.json";

fn load_tasks() -> Vec<Task> {
    let mut file = OpenOptions::new().read(true).open(FILE_PATH).unwrap_or_else(|_| {
        OpenOptions::new().write(true).create(true).open(FILE_PATH).unwrap()
    });

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    if content.is_empty() {
        vec![]
    } else {
        serde_json::from_str(&content).unwrap()
    }
}

fn save_tasks(tasks: &Vec<Task>) {
    let content = serde_json::to_string(tasks).unwrap();
    let mut file = OpenOptions::new().write(true).truncate(true).open(FILE_PATH).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

pub fn add_task(description: &str) {
    let mut tasks = load_tasks();
    let id = tasks.len() as u32 + 1;
    let task = Task { id, description: description.to_string(), completed: false };
    tasks.push(task);
    save_tasks(&tasks);
    println!("Task added.");
}

pub fn list_tasks() {
    let tasks = load_tasks();
    for task in tasks {
        println!("{}: {} [{}]", task.id, task.description, if task.completed { "x" } else { " " });
    }
}

pub fn remove_task(id: u32) {
    let mut tasks = load_tasks();
    tasks.retain(|task| task.id != id);
    save_tasks(&tasks);
    println!("Task removed.");
}

pub fn complete_task(id: u32) {
    let mut tasks = load_tasks();
    if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
        task.completed = true;
    }
    save_tasks(&tasks);
    println!("Task completed.");
}
