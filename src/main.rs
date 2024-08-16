use std::fs::OpenOptions;
use std::fs::File;
use std::fs::read_to_string;
use std::io::Write;
use std::env;
use std::path::Path;

static FILE_NAME: &str = "TaskList.txt";

pub fn display_menu() {
    println!("1. Add a new task");
    println!("2. Delete the task");
    println!("3. Mark task complete");
    println!("4. Show the tasks");
    println!("5. Quit");
}

pub fn strikethrough(text: &mut String) -> String {
    let mut result: String = String::new();
    for t in text.chars() {
        result.push_str("\u{0336}");
        result.push_str(&t.to_string());
    }
    result
}

pub fn file_write(task: &mut String) {
    // Write the new task added to the file
    let mut file = OpenOptions::new()
                .read(true)
                .append(true)
                .write(true)
                .create(true)
                .open(FILE_NAME)
                .expect("Could not add task!!");
    file.write_all(task.as_bytes()).expect("Write failed");
    let _ = file.write_all(b"\n");
    println!("Task {} added successfully", task);
} 

pub fn clear_todo() {
    std::fs::remove_file(FILE_NAME).expect("Could not clear tasks");
    println!("To-Do pile cleared!!!");
}

pub fn file_read() {
    // For showing all the tasks which are added
    if Path::new(FILE_NAME).is_file() {
        println!("--------------------------------------------------------------------------------");
        for (i, line) in read_to_string(FILE_NAME).unwrap().lines().enumerate() {
            println!("{}) {}", i+1, line);
        }
        println!("--------------------------------------------------------------------------------");
    }
    else {
        println!("No tasks in To-Do");
    }
}

pub fn delete_task(task_no: i32) {
    // remove the task from file take the task number as input
    let mut content: Vec<String> = Vec::new();
    if Path::new(FILE_NAME).is_file() {
        for (i, line) in read_to_string(FILE_NAME).unwrap().lines().enumerate() {
            if i+1 == task_no as usize {
                continue
            }
            content.push(line.to_string());
        }
        let mut file = File::create(FILE_NAME).expect("Could not open file");
        for cont in content {
            file.write_all(cont.as_bytes()).expect("Write Failed");
            let _ = file.write_all(b"\n");
        }
        println!("Deleted task {} successfully", task_no);
    }
    else {
        println!("No tasks in To-Do to remove")
    }
}

pub fn mark_complete(task_no: i32) {
    // strikethrough the task which is completed
    let mut content: Vec<String> = Vec::new();
    for (i, line) in read_to_string(FILE_NAME).unwrap().lines().enumerate() {
        if i+1 == task_no as usize {
            content.push(strikethrough(&mut line.to_string()));
        }
        else {
            content.push(line.to_string());
        }
    }
    let mut file = File::create(FILE_NAME).expect("Could not open file");
    for cont in content {
        file.write_all(cont.as_bytes()).expect("Write Failed");
        let _ = file.write_all(b"\n");
    }
    println!("Completed task {} successfully", task_no);
}

pub fn interactive() {
    println!("Welcome to the TO-DO list app!!!");
    loop {
        let mut input: String = String::new();
        display_menu();
        println!("Please choose the operation: ");
        std::io::stdin().read_line(&mut input).unwrap();
        // let input: i32 = input.trim().parse().unwrap();
        match input.trim().parse() {
            Ok(1) => {
                println!("Enter the name of the task: ");
                let mut task: String = String::new();
                std::io::stdin().read_line(&mut task).unwrap();
                file_write(&mut task);
            }
            Ok(2) => {
                println!("Enter the task number: ");
                let mut task_no: String = String::new();
                std::io::stdin().read_line(&mut task_no).unwrap();
                delete_task(task_no.trim().parse().unwrap());
            }
            Ok(3) => {
                println!("Enter the task number: ");
                let mut task_no: String = String::new();
                std::io::stdin().read_line(&mut task_no).unwrap();
                mark_complete(task_no.trim().parse().unwrap());
            }
            Ok(4) => {
                file_read();
            }
            Ok(5) => {
                break;
            }
            Err(_) | _ => { println!("Invalid input!!Please try again"); }
        }
    }
}

fn main() {
    // Try to take the inputs from command line rather than having interactive application
    // Examples could be, "ToDoList add <task name>", "ToDoList show", "ToDoList remove <task no>", ToDoList complete <task no>
    let mut args = env::args().skip(1);
    
    while let Some(arg) = args.next() {
        match &arg[..] {
            // "-h" | "help" => {
            //     help();
            // }
            // "version" => {
            //     println!("{} {}", prog().unwrap_or_default(), VERSION);
            // }
            "-a" | "add" => {
                if let Some(task_name) = args.next() {
                    let mut task = task_name;
                    file_write(&mut task);
                }
                else {
                    println!("No task specified for add");
                }
            }
            "-r" | "remove" => {
                if let Some(task_no) = args.next() {
                    let task = task_no;
                    delete_task(task.trim().parse().unwrap());
                }
                else {
                    println!("No task number provided to remove");
                }
            }
            "-c" | "complete" => {
                if let Some(task_no) = args.next() {
                    let task = task_no;
                    mark_complete(task.trim().parse().unwrap());
                }
                else {
                    println!("No task number provided to complete");
                }
            }
            "-s" | "show" => {
                file_read();
            }
            "-cl" | "clear" => {
                clear_todo();
            }
            "-i" | "interactive" => {
                interactive();
            }
            _ => {
                println!("Unknown argument");
            }
        }
    }
}
