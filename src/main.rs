use std::fs::OpenOptions;
use std::io::{self, BufRead, Write};

fn main() {
    let mut args = std::env::args();
    args.next(); // Skip the first argument (the program name)

    match args.next().as_deref() {
        Some("add") => add_task(args.collect()),
        Some("list") => list_tasks(),
        Some("remove") => remove_task(args.collect()),
        _ => eprintln!("Usage: task <add|list|remove>"),
    }
}

fn add_task(task: Vec<String>) {
    let task = task.join(" ");
    let mut file = OpenOptions::new().append(true).open("tasks.txt").expect("Could not open tasks file");
    writeln!(file, "{}", task).expect("Could not write to tasks file");
    println!("Task added: {}", task);
}

fn list_tasks() {
    let file = OpenOptions::new().read(true).open("tasks.txt").expect("Could not open tasks file");
    let reader = io::BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.expect("Could not read line from tasks file");
        println!("{}. {}", index + 1, line);
    }
}

fn remove_task(args: Vec<String>) {
    if args.is_empty() {
        eprintln!("Usage: task remove <task number>");
        return;
    }

    let task_num: usize = args[0].parse().expect("Invalid task number");
    let file = OpenOptions::new().read(true).open("tasks.txt").expect("Could not open tasks file");
    let reader = io::BufReader::new(file);
    let tasks: Vec<_> = reader.lines().collect();

    if task_num == 0 || task_num > tasks.len() {
        eprintln!("Invalid task number");
        return;
    }

    let task = &tasks[task_num - 1];
    println!("Removing task: {}", task.as_ref().expect("Invalid task"));
    let mut file = OpenOptions::new().write(true).truncate(true).open("tasks.txt").expect("Could not open tasks file for writing");
    for (index, line) in tasks.into_iter().enumerate() {
        if index != task_num - 1 {
            writeln!(file, "{}", line.expect("Could not read line")).expect("Could not write line");
        }
    }
}
