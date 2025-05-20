
use args::{Args, Commands};
use clap::{Parser};


mod args;
mod implementation;
mod sql;

fn main(){
    let args = Args::parse();

    let command = (args.command).to_lowercase();
    let index = args.index;
    let tasks = args.tasks;

    let action = match command.as_str(){
        "add" => Commands::Add(tasks),
        "edit" => Commands::Edit(index, tasks),
        "list" => Commands::List,
        "listdone" => Commands::ListDone,
        "listundone" => Commands::ListUnDone,
        "done" => Commands::Done(index),
        "rm" => Commands::Rm(index),
        "reset" => Commands::Reset,
        "sort" => Commands::Sort,
        _ => {
            println!("Invalid command. Use --help for more information.");
            return;
        }
    };

    match action {
        Commands::Add(tasks) => {
            if let Err(e) = implementation::add(tasks) {
                println!("Error: {}", e);
            }
        },
        Commands::Edit(index, task) => {
            if let Err(e) = implementation::edit(index, &task) {
                println!("Error: {}", e);
            }
        },
        Commands::List => {
            if let Err(e) = implementation::list() {
                println!("Error: {}", e);
            }
        },
        Commands::ListDone => {
            if let Err(e) = implementation::list_done() {
                println!("Error: {}", e);
            }
        },
        Commands::ListUnDone => {
            if let Err(e) = implementation::list_undone() {
                println!("Error: {}", e);
            }
        },
        Commands::Done(index) => {
            if let Err(e) = implementation::done(vec![index]) {
                println!("Error: {}", e);
            }
        },
        Commands::Rm(index) => {
            if let Err(e) = implementation::rm(vec![index]) {
                println!("Error: {}", e);
            }
        },
        Commands::Reset => {
            if let Err(e) = implementation::reset() {
                println!("Error: {}", e);
            }
        },
        Commands::Sort => {
            if let Err(e) = implementation::sort() {
                println!("Error: {}", e);
            }
        },
    }


}
