/*

Todo
Refactor the existing functions, especially main
Use handle_add as a template for the other functions to modify todo.json

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

const _VALID_LIST_ARGUMENTS: [&str; 3] = ["done", "todo", "in-progress"];

fn main() {
    let file = Path::new("todo.json");
    if file.exists() {
        let is_empty_file = fs::metadata(file).map(|data| data.len() == 0).unwrap();
        let command_line_arguments: Vec<String> = env::args().skip(1).collect();

        if is_empty_file {
            create_blank_todo_list();
        } else {
            validate_todo_list_file();
        }

        match valid_first_argument(&command_line_arguments) {
            Ok(()) => handle_arguments(command_line_arguments),
            Err(error_message) => println!("{error_message}"),
        }
    } else {
        println!("ERROR: Missing todo.json in current directory. Create it and try again");
    }
}

// END MAIN

fn valid_first_argument(arguments: &[String]) -> Result<(), &str> {
    if arguments.is_empty() {
        Err("Error: Must provide at least one argument")
    } else {
        let first_arg = arguments[0].to_lowercase();
        if VALID_FIRST_ARGUMENTS.contains(&first_arg.as_str()) {
            Ok(())
        } else {
            Err("Error: Invalid first argument to this program: {first_arg}")
        }
    }
}

fn handle_arguments(arguments: Vec<String>) {
    let first_arg = &arguments[0].to_lowercase();
    match first_arg.as_str() {
        "add" => handle_add(&arguments),
        "update" => handle_update(&arguments),
        "delete" => handle_delete(arguments),
        "mark-in-progress" => handle_mark_in_progress(arguments),
        "mark-done" => handle_mark_done(arguments),
        "handle_list" => handle_list(arguments),
        _ => println!("Error: Invalid argument provided: {first_arg}"),
    }
}

fn create_blank_todo_list() {
    let default_todo_list = TodoList { tasks: vec![] };
    let json = serde_json::to_string_pretty(&default_todo_list).unwrap();
    fs::write("todo.json", json).expect("Error: Failed to write to todo.json");
}

// The file may exist and may not be empty, but it may have incorrect data in it. It needs to be in the right format, mentioned above
fn validate_todo_list_file() {}

fn handle_add(arguments: &[String]) {
    let contents = fs::read_to_string("todo.json").unwrap();
    let mut todo_list: TodoList = serde_json::from_str(&contents).unwrap();
    let id = todo_list.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    let task_name = arguments[1..].join(" ");
    if todo_list.tasks.iter().any(|t| t.name == task_name) {
        println!("Error: A task with this name already exists: {task_name}");
    } else {
        todo_list.tasks.push(Task {
            id,
            name: task_name,
            status: "todo".to_string(),
        });
        let updated_todo_list = serde_json::to_string_pretty(&todo_list).unwrap();
        fs::write("todo.json", updated_todo_list).expect("Error: Failed to write to todo.json");
    }
}

fn handle_update(arguments: &[String]) {
    // let contents = fs::read_to_string("todo.json").unwrap();
    // let mut todo_list: TodoList = serde_json::from_str(&contents).unwrap();
    // Looks like most of the functions will need to read the file contents and make a todo_list. This could be moved into its own function to avoid duplication

    // Update is done via passing in the id
    // Need to check that the value at arguments[1] is a valid number
    // If it is a valid number, need to check that it exists in the todo.json file in the tasks array
    // If it does, need to then check that the update name is not empty (They need to pass in something to update it with)

    //if todo_list.tasks.iter().any(|t| t.id == task_id) {
    // Update task
    //} else {
    //println!("Error: There is no task with the ID: {task_id}");
    //}
}

fn handle_delete(_arguments: Vec<String>) {}

fn handle_mark_in_progress(_arguments: Vec<String>) {}

fn handle_mark_done(_arguments: Vec<String>) {}

// TODO This should check for additional arguments after list command, such as done, todo , in-progress and pass them off accordingly.
fn handle_list(_arguments: Vec<String>) {}

fn _handle_list_done(_arguments: Vec<String>) {}

fn _handle_list_todo(_arguments: Vec<String>) {}

fn _handle_list_in_progress(_arguments: Vec<String>) {}
