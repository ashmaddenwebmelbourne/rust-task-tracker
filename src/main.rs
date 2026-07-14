/*

To do list:

Get the add function to work and add a task. Can reuse the same logic for reading/writing/updating the file and fields

*/

use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct Task {
    id: u64,
    name: String,
    status: String,
}

#[derive(Serialize, Deserialize)]
struct TodoList {
    tasks: Vec<Task>,
}

const VALID_FIRST_ARGUMENTS: [&str; 6] = [
    "add",
    "update",
    "delete",
    "mark-in-progress",
    "mark-done",
    "list",
];

const VALID_LIST_ARGUMENTS: [&str; 3] = ["done", "todo", "in-progress"];

fn main() {
    let todo_file = Path::new("todo.json");
    if todo_file.exists() {
        let todo_file_is_empty = fs::metadata(&todo_file).map(|metadata| metadata.len() == 0);
        let command_line_arguments: Vec<String> = env::args().skip(1).collect();

        match todo_file_is_empty {
            Ok(true) => create_blank_todo_list(),
            _ => validate_todo_list_file(),
        }

        match validate_first_argument(&command_line_arguments) {
            Ok(()) => handle_arguments(command_line_arguments),
            Err(e) => println!("{}", e),
        }
    } else {
        println!("ERROR: Missing todo.json in current directory. Create it and try again");
    }
}

fn validate_first_argument(arguments: &[String]) -> Result<(), &str> {
    if arguments.is_empty() {
        Err("Error: Must provide at least one argument")
    } else {
        let first_arg = arguments[0].to_lowercase();
        match VALID_FIRST_ARGUMENTS.contains(&first_arg.as_str()) {
            true => Ok(()), // command is add, update, delete, mark-in-progress, mark-done or list
            false => Err("Error: Invalid first argument to this program"),
        }
    }
}

fn handle_arguments(arguments: Vec<String>) {
    let first_arg = &arguments[0].to_lowercase();
    match first_arg.as_str() {
        "add" => handle_add(arguments),
        "update" => handle_update(arguments),
        "delete" => handle_delete(arguments),
        "mark-in-progress" => handle_mark_in_progress(arguments),
        "mark-done" => handle_mark_done(arguments),
        "handle_list" => handle_list(arguments),
        _ => println!("Error: Invalid argument provided: {}", first_arg),
    }
}

fn create_blank_todo_list() {
    let default_todo_list = TodoList { tasks: vec![] };
    let json = serde_json::to_string_pretty(&default_todo_list).unwrap();
    fs::write("todo.json", json).expect("Error: Failed to write to todo.json");
}

// The file may exist and may not be empty, but it may have incorrect data in it. It needs to be in the right format, mentioned above
fn validate_todo_list_file() {}

// This doesn't work properly
fn handle_add(arguments: Vec<String>) {
    let todo_file_contents = fs::read_to_string("todo.json").unwrap();
    let mut todo_list: TodoList = serde_json::from_str(&todo_file_contents).unwrap();
    let task_name = &arguments[1];
    let new_id = todo_list
        .tasks
        .iter()
        .map(|task| task.id)
        .max()
        .unwrap_or(0);
    if todo_list
        .tasks
        .iter()
        .any(|task| task.name == String::from(task_name))
    {
        println!("Error: A task with this name already exists: {}", task_name);
    } else {
        todo_list.tasks.push(Task {
            id: new_id,
            name: arguments[1].clone(),
            status: String::from("todo"),
        });
    }
}

fn handle_update(arguments: Vec<String>) {}

fn handle_delete(arguments: Vec<String>) {}

fn handle_mark_in_progress(arguments: Vec<String>) {}

fn handle_mark_done(arguments: Vec<String>) {}

// TODO This should check for additional arguments after list command, such as done, todo , in-progress and pass them off accordingly.
fn handle_list(arguments: Vec<String>) {}

fn handle_list_done(arguments: Vec<String>) {}

fn handle_list_todo(arguments: Vec<String>) {}

fn handle_list_in_progress(arguments: Vec<String>) {}
