use std::io;

struct Task {
    task: String,
    done: bool,
}

impl Task {
    fn new(task: String) -> Task {
        Task { task, done: false }
    }

    fn complete(&mut self) {
        self.done = true;
    }
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("choose an option");
        println!("1. add task");
        println!("2. show tasks");
        println!("3. complete task");
        println!("4. quit");

        let mut choice = String::new();
        io::stdio().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                println!("enter task description:");
                let mut task_text = String::new();
                io::stdin().read_line(&mut task_text).unwrap();
                let task_text = task_text.trim().to_string();
                let task = Task::new(task_text);
                tasks.push(task);
                println!("task added");
            }

            "2" => {
                println!("your tasks:");
                for (i, t) in tasks.iter().enumerate() {
                    println!("{}. {} [{}]", t.task, if t.done {"done"} else {"not done"});
                }
            }

            "3" => {
                println!("enter task number to complete:");
                let mut num_str = String::new();
                io::stdin().read_line(&mut num_str).unwrap();

                if let Ok(num) = num_str.trim().parse::<usize>() {
                    if num > 0 && num <= tasks.len() {
                        tasks[num - 1].complete();
                        println!("task marked as done");
                    } else {
                        println!("invalid task number");
                    }
                } else {
                    println!("enter a valid number");
                }
            }
            "4" => {
                println!("all done");
                break;
            }
            _ => println!("invalid option, try again"),
        }
    }
}
