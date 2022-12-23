use clap::{Parser, Subcommand};
use tokio;

mod model;
mod utils;
mod server;

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
    Serve { bind_addr: Option<String> },
    List,
    Add {name: String },
    Remove {index: usize }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let todo_file = cli.todo_file.unwrap_or(TODO_FILE.to_string());

    let mut data = model::TodoList::load(&todo_file);

    let res = match &cli.command {
        Some(Commands::Serve { bind_addr}) =>  {
            let default_addr = "0.0.0.0:8080".to_string();
            let actual_addr: &str = match bind_addr {
                Some(addr) => &addr,
                None => &default_addr
            };
            server::start_server(&actual_addr, data).await
            .map_err(|e| e.to_string())
        },

        Some(Commands::List) => {
            for e in data.entries() {
                println!("- {:?}", e)
            }
            Ok(())
        },

        Some(Commands::Add { name }) => {
            data.add_entry(model::TodoEntry { title: name.clone(), body: "Some body".to_string() } );
            data.save(&todo_file)
        }

        Some(Commands::Remove { index }) => {
            // Here we cannot use ? because we are not in a function???
            match data.remove_entry(*index) {
                Ok(()) => data.save(&todo_file),
                Err(err) => Err(err)
            }
        }

        None => {
            println!("Specify a command");
            Ok(())
        }
    };

    match res {
        Ok(_) => (),
        Err(e) => println!("{}", e)
    }

   
}
