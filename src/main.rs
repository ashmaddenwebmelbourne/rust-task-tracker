/*
Todo
Complete handle_delete function
*/

use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
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

// Checks that the first command line argument passed to the program is add, update, delete, mark-in-progress, mark-done or list. Errors otherwise.
fn valid_first_argument(arguments: &[String]) -> Result<(), &str> {
    if arguments.is_empty() {
        Err("Error: Must provide at least one argument")
    } else {
        let first_arg = arguments[0].to_lowercase();
        if VALID_FIRST_ARGUMENTS.contains(&first_arg.as_str()) {
            Ok(())
        } else {
            Err("Error: Invalid first argument to this program")
        }
    }
}

// If the first command line argument is a valid argument, this function hands the command line arguments off to their respective handlers for execution.
fn handle_arguments(arguments: Vec<String>) {
    let first_arg = &arguments[0].to_lowercase();
    match first_arg.as_str() {
        "add" => handle_add(&arguments),
        "update" => handle_update(&arguments),
        "delete" => handle_delete(&arguments),
        "mark-in-progress" => handle_mark_in_progress(&arguments),
        "mark-done" => handle_mark_done(arguments),
        "handle_list" => handle_list(arguments),
        _ => println!("Error: Invalid argument provided: {first_arg}"),
    }
}

// If todo.json is empty, this function creates the default template, which contains an empty tasks array, to begin putting tasks into
fn create_blank_todo_list() {
    let default_todo_list = TodoList { tasks: vec![] };
    let json = serde_json::to_string_pretty(&default_todo_list).unwrap();
    fs::write("todo.json", json).expect("Error: Failed to write to todo.json");
}

// If todo.json is not empty, this function checks that the file is in the correct format.
fn validate_todo_list_file() {}

// Reads the todo.json file and returns a parsed data structure representing the JSON file
fn read_todo() -> TodoList {
    let contents = fs::read_to_string("todo.json").unwrap();
    serde_json::from_str(&contents).unwrap()
}

// Takes the updated, parsed data structure representing the JSON file and writes to todo.json
fn write_todo(todo_list: &TodoList) {
    let updated_todo_list = serde_json::to_string_pretty(&todo_list).unwrap();
    fs::write("todo.json", updated_todo_list).expect("Error: Failed to write to todo.json");
}

// Adds a task to todo.json
fn handle_add(arguments: &[String]) {
    let mut todo_list = read_todo();
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
        write_todo(&todo_list);
        println!("Task added to todo list");
    }
}

// Update a task name via passing in its ID and a new name
fn handle_update(arguments: &[String]) {
    let mut todo_list = read_todo();
    let task_id: u64 = arguments[1].parse().unwrap_or_default();
    if task_id == 0 {
        println!("Error: You must provide the ID number of the task you want to update");
    } else if let Some(task) = todo_list.tasks.iter_mut().find(|t| t.id == task_id) {
        let updated_name = arguments[2..].join(" ");
        task.name = updated_name;
        write_todo(&todo_list);
        println!("Updated task {task_id} successfully!");
    } else {
        println!("Error: There is no task that has the ID: {task_id}");
    }
}

// Delete a task via passing in its ID
fn handle_delete(arguments: &[String]) {
    let mut todo_list = read_todo();
    let task_id: u64 = arguments[1].parse().unwrap_or_default();
    let task_is_in_file = todo_list.tasks.iter().any(|t| t.id == task_id);
    if task_id == 0 {
        println!("Error: You must provide the ID number of the task you want to delete"); // Something other than a number was passed in 
    } else if task_is_in_file {
        // Removes task with matching ID
        todo_list.tasks.retain(|t| t.id != task_id);
        // Adjust the ID's of the tasks to account for task deletion (we want the ID's to always be in sequential order, with no gaps)
        for (i, task) in todo_list.tasks.iter_mut().enumerate() {
            task.id = i as u64 + 1;
        }
        write_todo(&todo_list);
        println!("Task {task_id} successfully removed");
    } else {
        println!("Error: There is no task that has the ID: {task_id}");
    }
}

// Update the status of a task to 'in-progress' via passing in its ID
fn handle_mark_in_progress(arguments: &[String]) {
    let mut todo_list = read_todo();
    let task_id: u64 = arguments[1].parse().unwrap_or_default();
    let task_is_in_file = todo_list.tasks.iter().any(|t| t.id == task_id);
    if task_id == 0 {
        println!(
            "Error: You must provide the ID number of the task you want to mark as 'in progress'"
        );
    } else if task_is_in_file {
        for task in &mut todo_list.tasks {
            if task.id == task_id {
                task.status = String::from("in-progress");
                break;
            }
        }
        write_todo(&todo_list);
        println!("Status for task {task_id} updated successfully");
    } else {
        println!("Error: There is no task that has the ID: {task_id}");
    }
}

fn handle_mark_done(_arguments: Vec<String>) {}

// TODO This should check for additional arguments after list command, such as done, todo , in-progress and pass them off accordingly.
fn handle_list(_arguments: Vec<String>) {}

fn _handle_list_done(_arguments: Vec<String>) {}

fn _handle_list_todo(_arguments: Vec<String>) {}

fn _handle_list_in_progress(_arguments: Vec<String>) {}
