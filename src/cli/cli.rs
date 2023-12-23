use crate::controllers::task::{TaskController, TaskError};
use crate::hex_to_rgb;
use color_print::cprintln;
use std::process;
use speedate::Date;


use crate::backend::serve_server;
use crate::task::task::Task;
use crate::task::task_instance::draw_chart;
use colored::*;

#[warn(dead_code)]
pub struct Query<'a> {
    pub filepath: &'a str,
    args: &'a [String],
}

pub struct Commands<'a> {
    all: [&'a str; 10],
    pub arr: [[&'a str; 4]; 6],
}

impl<'a> Commands<'a> {
    pub fn new() -> Commands<'a>{
        let all = [
            "",
            "create", "start", "stop", "show",
            "task 'name'", "server", "tasks", "status", "graph",
        ];

        let arr = [
            [ all[1], all[2], all[3], all[4] ],
            [ all[5], all[5], all[5], all[0] ],
            [ all[0], all[0], all[6], all[0] ],
            [ all[0], all[0], all[0], all[7] ],
            [ all[0], all[0], all[0], all[8] ],
            [ all[0], all[0], all[0], all[9] ]
        ];

        Commands {
            all: all,
            arr
        }

    }
}

impl<'a> Query<'a> {
    pub fn new(args: &'a Vec<String>) -> Query<'a> {
        let filepath = &args[0];
        let last_index = filepath.rfind("/").unwrap();
        let filepath = &filepath[0..last_index];
        Query {
            filepath: &filepath,
            args: &args[1..],
        }
    }

    pub fn process_args(&self) {
        if self.args.len() == 0 {
            cprintln!("⚠️  <yellow>No arguments provided!</>");
            help();
        }
        println!("{}", self.filepath);
        let command: &str = &self.args[0];
        match command {
            "create" => ArgsHandeler::command_create(&self, 1),
            "show" => ArgsHandeler::command_show(&self, 1),
            "start" => ArgsHandeler::command_start(&self, 1),
            "stop" => ArgsHandeler::command_stop(&self, 1),
            _ => ArgsHandeler::command_not_found(self, 0),
        };
    }

    // min inclusive, max exclusive!
    fn format_args(&self, from: usize, to: usize) -> String {
        let mut final_string: String = String::new();
        for i in from..to {
            final_string.push_str(&format!("{} ", self.args[i]));
        }

        final_string
    }
}

pub struct ArgsHandeler;
impl ArgsHandeler {
    fn command_create(q: &Query, command_n: usize) {
        let command: &str = &q.args[command_n];
        let _ = match command {
            "task" => {
                let task_name: &str = &q.args[command_n + 1].trim_matches('"');
                let t: Task = Task::new(&task_name);
                let result: Result<Task, TaskError> = TaskController::create_task(t);
                match result {
                    Err(err) => {
                        let error_code: i8 = err.code;
                        match error_code {
                            -1 => cprintln!("<red>The task </>'{}'<red>, already exists!</>", task_name),
                            _ => todo!(),
                        }
                    }
                    Ok(_) => cprintln!("<green>The task </>'{}'<green>, has been successfully created!</>", task_name),
                    
                };
            }
            _ => {
                ArgsHandeler::command_not_found(q, command_n);
            }
        };
    }

    fn command_show(q: &Query, command_n: usize) {
        let command: &str;
        if q.args.len() < 2 {
            command = "";
        } else {
            command = &q.args[command_n];
        }

        let _ = match command {
            "tasks" => {
                let tasks = TaskController::get_tasks();
                match tasks.len() > 0 {
                    false => println!("There are no tasks loaded..."),
                    true => {
                        for t in &tasks {
                            println!("{:?}", t);
                        }
                    }
                }
            }
            "graph" => {
                let date: String;
                if &q.args.len() > &(command_n+1) {
                    date = q.args[command_n+1].to_string();
                } else {
                    date = Date::today(0).unwrap().to_string();
                }
                let tasks = TaskController::get_date_tasks(date);
                let exit_code: i8 = draw_chart(&tasks);
                if exit_code == -1{
                    println!("There are no tasks loaded for this day...")
                }
            }
            "status" => {
                let tasks = TaskController::get_running_tasks();
                match tasks.len() > 0 {
                    false => println!("There is no tasks running!"),
                    true => {
                        for t in &tasks {
                            let rgb_color = hex_to_rgb(&t.task.color);
                            println!("{}", t.task.name.truecolor(rgb_color.0, rgb_color.1, rgb_color.2));
                            println!("  + started at {}", t.start_time);
                            println!("  + {} mins", t.duration);
                            println!("");
                        }
                    },
                };
            }
            _ => ArgsHandeler::command_not_found(q, command_n),
        };
    }

    fn command_start(q: &Query, command_n: usize) {
        let command: &str = &q.args[command_n];
        let _ = match command {
            "task" => {
                let task_name: &str = &q.args[command_n + 1].trim_matches('"');
                let result: Result<(), TaskError> = TaskController::start_task(&task_name);
                match result {
                    Err(err) => {
                        let error_code: i8 = err.code;
                        match error_code {
                            -1 => cprintln!("<red>The task </>'{}'<red>, doesn't exist!</>", task_name),
                            -2 => cprintln!("⚠️ <yellow>The task </>'{}'<yellow> is already running</>", task_name),
                            _ => todo!()
                        }
                    }
                    Ok(_) => {
                        cprintln!("✔️ <green>The task </>'{}'<green>, has started successfully!</>", task_name);
                        notify("start_task_notify");
                    }
                };
            }
            "server" => {
                let _ = serve_server();
            }
            _ => ArgsHandeler::command_not_found(q, command_n),
        };
    }

    fn command_stop(q: &Query, command_n: usize) {
        let command: &str = &q.args[command_n];
        let _ = match command {
            "task" => {
                let task_name: &str = &q.args[command_n + 1];
                let result: Result<(), TaskError> = TaskController::stop_task(&task_name);
                match result {
                    Err(err) => {
                        let error_code = err.code;
                        match error_code {
                            -1 => cprintln!("<red>The task </>'{}'<red>, doesn't exist!</>", task_name),
                            -2 => cprintln!("⚠️ <yellow>The task </>'{}'<yellow> is not running</>", task_name),
                            _ => todo!()
                        }
                    }
                    Ok(_) => {
                        cprintln!("🛑 The task '{}', has stoped...", task_name);
                        notify("stop_task_notify");
                    }
                }
            }
            _ => ArgsHandeler::command_not_found(q, command_n),
        };
    }

    fn command_not_found(q: &Query, command_n: usize) {
        if command_n < q.args.len() {
            cprintln!(
                "{}<red><u>\"{}\"</>: command not found!</red>",
                q.format_args(0, command_n),
                q.args[command_n]
            );
        }
        println!();
        let basic_command = &q.args[0];
        let c = Commands::new();
        let index = c.arr[0].iter().position(|&r| r.eq(basic_command)).unwrap();

        for ii in 1..c.arr.len() {
            if "".eq(c.arr[ii][index]) {
                continue;
            }
            println!("    - {}", c.arr[ii][index]);
        }


        process::exit(1);
    }
}

fn notify(action: &str) {
    let url = format!("http://192.168.1.40:8080/{}", action);

    match reqwest::blocking::Client::new().post(url).send() {
        Ok(response) => {
            if response.status().is_success() {
                println!("\tℹ️ notification sent successfully");
            }
        }
        Err(_) => {}
    }
}

fn help() {
    let c = Commands::new();
    println!("Basic commands: \n");
    let basic_commands = c.arr.len();
    for i in 0..basic_commands {
        println!("  {}", c.arr[0][i]);

        for ii in 1..c.arr.len() {
            if "".eq(c.arr[ii][i]) {
                continue;
            }
            println!("    - {}", c.arr[ii][i]);
        }

        println!();
    }
    
    process::exit(1);
}
