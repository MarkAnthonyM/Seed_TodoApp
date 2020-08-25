use std::env;
use backend::db::{ create_task, delete_task, establish_connection, query_task, update_task_status };

fn help() {
    println!("subcommands:");
    println!("  delete<id>: delete task");
    println!("  finish<id>: update task status as done");
    println!("  new<title><done>: create a new task");
    println!("  show: show todo items");
}

// Delete task from database
fn remove_task(args: &[String]) {
    if args.len() < 1 {
        println!("delete: missing ID argument");
        help();
        return;
    }

    let conn = establish_connection();

    for task in args.iter() {
        delete_task(&conn, &task);
    }
}

fn finish_task(args: &[String]) {
    // Need to handle case with multiple arguments
    if args.len() < 1 {
        println!("finish: missing argument");
        help();
        return;
    }

    let conn = establish_connection();
    
    for task in args.iter() {
        update_task_status(&conn, &task);
    }
}

// subcommand handler for task creation
fn new_task(args: &[String]) {
    if args.len() < 2 {
        // Need better user feedback
        println!("new: requires both <title> and <done> arguments");
        help();
        return;
    }

    let conn = establish_connection();
    // Need to adjust create_task function for case insensitivity
    create_task(&conn, &args[0], &args[1].parse::<bool>().expect("Needs to be a lowercase true or false statement"));
}

// subcommand handler for task query
fn show_tasks(args: &[String]) {
    if args.len() > 0 {
        println!("show: unexpected argument");
        help();
        return;
    }

    let conn = establish_connection();
    println!("TASKS\n-----");
    for task in query_task(&conn) {
        println!("Title: {}, Done: {}\n", task.title, task.done);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check for correct number of arguments, return help error if true
    if args.len() < 2 {
        help();
        return;
    }

    let subcommand = &args[1];

    // Parse for valid arguments, return help prompt for invalid input
    match subcommand.as_ref() {
        "delete" => remove_task(&args[2..]),
        "finish" => finish_task(&args[2..]),
        "new" => new_task(&args[2..]),
        "show" => show_tasks(&args[2..]),
        _ => help(),
    }
}