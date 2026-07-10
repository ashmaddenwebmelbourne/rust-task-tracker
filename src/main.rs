use std::env;
use std::path::Path;

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
        let command_line_arguments: Vec<String> = env::args().skip(1).collect();
        let (is_valid_first_argument, error_message) =
            validate_first_argument(&command_line_arguments);
        match is_valid_first_argument {
            true => handle_arguments(command_line_arguments), // Each command will validate itself later based on its own requirements, we just need a valid first command to proceed
            false => println!("{error_message}"),
        }
    } else {
        println!("ERROR: Missing todo.json in current directory. Create it and try again");
    }
}

// Refactor to Result<(), &str> type for better success/error retrns
fn validate_first_argument(arguments: &[String]) -> (bool, &str) {
    if arguments.is_empty() {
        (false, "Error: Must provide at least one argument")
    } else {
        let first_arg = arguments[0].to_lowercase();
        match VALID_FIRST_ARGUMENTS.contains(&first_arg.as_str()) {
            true => (true, ""), // command is add, update, delete, mark-in-progress, mark-done or list
            false => (false, "Error: Invalid first argument to this program"),
        }
    }
}

// Routes command to respective function handler
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

// these handle the functionality of the todo list, validating each command and argument(s) as needed
// CHECK that arguments are correct for <arg> command
// UPDATE JSON file
fn handle_add(arguments: Vec<String>) {}

fn handle_update(arguments: Vec<String>) {}

fn handle_delete(arguments: Vec<String>) {}

fn handle_mark_in_progress(arguments: Vec<String>) {}

fn handle_mark_done(arguments: Vec<String>) {}

// TODO This should check for additional arguments after list command, such as done, todo , in-progress and pass them off accordingly.
fn handle_list(arguments: Vec<String>) {}

fn handle_list_done(arguments: Vec<String>) {}

fn handle_list_todo(arguments: Vec<String>) {}

fn handle_list_in_progress(arguments: Vec<String>) {}
