use std::fs::OpenOptions;
use std::io::Write;
use std::fs::File;
use std::fs::read_to_string;

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
                .open("TaskList.txt")
                .expect("Could not add task!!");
    file.write_all(task.as_bytes()).expect("Write failed");
    println!("Task {} added successfully", task);
} 

pub fn file_read() {
    // For showing all the tasks which are added
    // let mut file = OpenOptions::new()
    //             .read(true)
    //             .open("TaskList.txt")
    //             .expect("Could not show tasks!!");
    println!("--------------------------------------------------------------------------------");
    for (i, line) in read_to_string("TaskList.txt").unwrap().lines().enumerate() {
        println!("{}) {}", i+1, line);
    }
    println!("--------------------------------------------------------------------------------");
}

pub fn delete_task(task_no: i32) {
    // remove the task from file take the task number as input
    let mut content: Vec<String> = Vec::new();
    for (i, line) in read_to_string("TaskList.txt").unwrap().lines().enumerate() {
        if i+1 == task_no as usize {
            continue
        }
        content.push(line.to_string());
    }
    let mut file = File::create("TaskList.txt").expect("Could not open file");
    for cont in content {
        file.write_all(cont.as_bytes()).expect("Write Failed");
        let _ = file.write_all(b"\n");
    }
    println!("Deleted task {} successfully", task_no);
}

pub fn mark_complete(task_no: i32) {
    // strikethrough the task which is completed
    let mut content: Vec<String> = Vec::new();
    for (i, line) in read_to_string("TaskList.txt").unwrap().lines().enumerate() {
        if i+1 == task_no as usize {
            content.push(strikethrough(&mut line.to_string()));
        }
        else {
            content.push(line.to_string());
        }
    }
    let mut file = File::create("TaskList.txt").expect("Could not open file");
    for cont in content {
        file.write_all(cont.as_bytes()).expect("Write Failed");
        let _ = file.write_all(b"\n");
    }
    println!("Completed task {} successfully", task_no);
}

fn main() {
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
        println!("This is the input: {}", input);
    }
}
