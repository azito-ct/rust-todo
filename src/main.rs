use clap::{Parser, Subcommand};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct TodoEntry {
    title: String,
    body: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TodoList {
    entries: Vec<TodoEntry>
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { entries: Vec::new() }
    }
}

fn save<T: Serialize>(dest: &str, data: &T) -> Result<(), String> {
    let path = Path::new(dest);
    let mut file = File::create(path).map_err(|e| e.to_string())?;
    let data = serde_json::to_string(data).map_err(|e| e.to_string())?;

    file.write_all(data.as_bytes()).map_err(|e| e.to_string())
}

fn load<T: DeserializeOwned>(source: &str) -> Result<Option<T>, String> {
    let path = Path::new(source);
    match File::open(path).map_err(|e| e.to_string()) {
        Ok(mut file) => {
            let mut buffer: Vec<u8> = Vec::new();
            file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;

            serde_json::from_slice::<T>(&buffer).map_err(|e| e.to_string())
            .map(Some)
        }

        Err(err) => Ok(None),
    }
}

const TODO_FILE: &str = "./todo.json";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    todo_file: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    List,
    Add {name: String },
    Remove {index: u8 }
}
fn main() {
    let cli = Cli::parse();

    let todo_file = cli.todo_file.unwrap_or(TODO_FILE.to_string());

    let mut data = match load::<TodoList>(&todo_file) {
        Ok(maybe_data) => maybe_data.unwrap_or(TodoList::new()),
        Err(error) => panic!("Error loading data: {}", error),
    };

    let res = match &cli.command {
        Some(Commands::List) => {
            for e in data.entries {
                println!("- {:?}", e)
            }
            Ok(())
        },

        Some(Commands::Add { name }) => {
            data.entries.push(TodoEntry { title: name.clone(), body: "Some body".to_string() } );
            save::<TodoList>(&todo_file, &data)
        }

        Some(Commands::Remove { index }) => 
            unimplemented!("implement remove"),

        None => {
            println!("Specify a command");
            Ok(())
        }
    };

    match res {
        Ok(_) => (),
        Err(e) => panic!("{}", e)
    }

   
}
