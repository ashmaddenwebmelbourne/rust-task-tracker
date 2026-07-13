use std::env;
use std::fs;
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
        let todo_file_is_empty = fs::metadata(&todo_file).map(|metadata| metadata.len() == 0);
        let command_line_arguments: Vec<String> = env::args().skip(1).collect();

        // Adds basic fields in JSON file so that all other functions behave as expected, if todo.json is empty
        match todo_file_is_empty {
            Ok(true) => create_blank_todo_list(),
            _ => validate_todo_list_file(), // Need to exit with error message if the todo list is in the wrong format
        }

        // Each command will validate itself later based on its own requirements, we just need a valid first command to proceed
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

// Handles the JSON file, ensuring it has the correct fields
// When called this should set the following default fields in the JSON file, ready to be used as a todo list

/*

Note that taskName is an example placeholder value

{
  "tasks": {
    "taskName": {
      "id": 1,
      "taskDescription": "Do something",
      "status": "todo"
    }
  }
}
*/

fn create_blank_todo_list() {
    println!("Ready to add fields to JSON file");

    // Add JSON fields here
}

// The file may exist and may not be empty, but it may have incorrect data in it. It needs to be in the right format, mentioned above
fn validate_todo_list_file() {}

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
