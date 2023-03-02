use chrono::{DateTime, Local};
use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

const INITIAL_DATE: &str = "2023-03-02 16:30:48.238795 -03:00";

fn get_number_weeks() -> (String, String, i64) {
    let initial_date: DateTime<Local> = INITIAL_DATE.parse().unwrap();
    let now: DateTime<Local> = Local::now();
    // let now: DateTime<Local> = Local::now();
    println!("Initial date: {}", initial_date);
    let diff = now - initial_date;
    let date_str: String = now.format("%Y-%m-%d %H:%M:%S").to_string();
    let mut date_split: Vec<&str> = date_str.split_whitespace().collect();
    return (
        date_split.remove(0).to_string(),
        date_split.pop().unwrap().to_string(),
        diff.num_weeks() + 1,
    );
}

fn handle_new_file(dir: &str, task: String, description: String) -> () {
    // format date
    let (date, hour, weeks) = get_number_weeks();
    let mut dir_path = format!("{}/Week-{}/", dir, weeks);
    let file_name = format!("{}-{}-{}.md", &task.replace(" ", "_"), &date, &hour);
    let to_write = format!(
        "# Task {}\n\n## Date {} Hour {}\n\n## Description:\n - {} ",
        &task, &date, &hour, &description
    );
    if !Path::new(&dir_path).try_exists().unwrap() {
        match std::fs::create_dir_all(&dir_path) {
            Err(why) => panic!("Couldn't create directory {}: {}", dir_path, why),
            Ok(_) => println!("Successfully created {}", dir_path),
        }
    }
    dir_path.push_str(&file_name);
    let file_path = Path::new(&dir_path);
    let mut file = match File::create(&file_path) {
        Err(why) => panic!("Couldn't create {}: {}", file_name, why),
        Ok(file) => file,
    };
    match file.write_all(to_write.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", file_name, why),
        Ok(_) => println!("Successfully wrote to {}", file_name),
    };
}

fn main() {
    println!("Add a new item to the 📒");
    let mut task = String::new();
    let mut description = String::new();
    let dir = "/Users/milosoria/projects/Trofica/work-log";
    let stdin = io::stdin();
    println!("✅ Enter a task: ");
    task = match stdin.read_line(&mut task) {
        Err(why) => panic!("Couldn't read input: {}", why),
        Ok(_) => task.trim_end().to_string(),
    };
    println!("💻 Enter a description: ");
    description = match stdin.read_line(&mut description) {
        Err(why) => panic!("Couldn't read input: {}", why),
        Ok(_) => description.trim_end().to_string(),
    };
    handle_new_file(dir, task, description);
}
