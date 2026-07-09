use std::env;

const VALID_STARTING_ARGUMENTS: [&str; 6] = [
    "add",
    "update",
    "delete",
    "mark-in-progress",
    "mark-done",
    "list",
];

const VALID_LIST_ARGUMENTS: [&str; 3] = ["done", "todo", "in-progress"];

// just checks that the argument format supplied is valid. doesn't validate further than that
fn is_valid_arguments(arguments: &Vec<String>) -> bool {
    // must have at least one argument
    if arguments.len() == 0 {
        false
    } else {
        let first_arg = arguments[0].to_lowercase();
        // must start with a valid starting argument
        if VALID_STARTING_ARGUMENTS.contains(&first_arg.as_str()) {
            if first_arg != "list" {
                true // first argument is add, update, delete, mark-in-progress or mark-done
            } else {
                if first_arg == "list" && arguments.len() == 1 {
                    true // first argument is list, by itself
                } else {
                    // if starting argument is list and there are following arguments, it expects done, todo or in-progress to follow
                    let list_arg = arguments[1].to_lowercase();
                    if VALID_LIST_ARGUMENTS.contains(&list_arg.as_str()) {
                        true // following argument is done, todo or in-progress 
                    } else {
                        false // not a valid list argument
                    }
                }
            }
        } else {
            false // not a valid starting argument
        }
    }
}

// once validated, this function determines what command should be executed, calls the required function
fn execute_argument() {}

// these handle the functionality of the todo list, validating each command and argument(s) as needed
fn handle_add() {}

fn handle_update() {}

fn handle_delete() {}

fn handle_mark_in_progress() {}

fn handle_mark_done() {}

fn handle_list() {}

fn handle_list_done() {}

fn handle_list_todo() {}

fn handle_list_in_progress() {}

fn main() {
    // Grabs the arguments, but not the program name
    let command_line_arguments: Vec<String> = env::args().skip(1).collect();

    println!("{}", is_valid_arguments(&command_line_arguments));
    println!("{:?}", command_line_arguments);

    // JSON file should be stored in same directory as executable. file called todo.json

    /*
    Pseudocode:

    PRESTEP 1: Check for todo.json file in same directory, return error if not found
    STEP 1: PROMPT user for input
    GET command line arguments
    CHECK if command line arguments are valid
    CALL execute_argument function
        IF argument is <arg>, call handle_<arg> function
            CHECK that arguments are correct for <arg> command
                UPDATE JSON file
                    GOTO STEP 1: PROMPT user for input
    */
}
