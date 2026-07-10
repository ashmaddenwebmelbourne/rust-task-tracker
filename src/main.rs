use std::env;
use std::path::Path;

const VALID_STARTING_ARGUMENTS: [&str; 6] = [
    "add",
    "update",
    "delete",
    "mark-in-progress",
    "mark-done",
    "list",
];

const VALID_LIST_ARGUMENTS: [&str; 3] = ["done", "todo", "in-progress"];

// Checks that first command provided is valid
fn is_valid_arguments(arguments: &Vec<String>) -> (bool, &str) {
    if arguments.len() == 0 {
        (false, "Error: Must provide at least one argument")
    } else {
        let first_arg = arguments[0].to_lowercase();
        match VALID_STARTING_ARGUMENTS.contains(&first_arg.as_str()) {
            true => (true, ""), // command is add, update, delete, mark-in-progress, mark-done or list
            false => (
                false,
                "Error: You must provide a valid argument to this program",
            ),
        }
    }
}

fn execute_argument(arguments: Vec<String>) {
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

// This should check for additional arguments after list command, such as done, todo , in-progress and pass them off accordingly.
fn handle_list(arguments: Vec<String>) {}

fn handle_list_done(arguments: Vec<String>) {}

fn handle_list_todo(arguments: Vec<String>) {}

fn handle_list_in_progress(arguments: Vec<String>) {}

fn main() {
    let todo_file = Path::new("todo.json");
    if todo_file.exists() {
        let command_line_arguments: Vec<String> = env::args().skip(1).collect();
        let (is_valid_arguments, error_message) = is_valid_arguments(&command_line_arguments);
        match is_valid_arguments {
            true => execute_argument(command_line_arguments),
            false => println!("{error_message}"),
        }
    } else {
        println!(
            "ERROR: You must have a todo.json file in the current directory to use. Create this file and then start the program again"
        );
    }
}
