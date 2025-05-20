use args::{Args, Commands};
use clap::{Parser};
use implementation::{add, done, edit, list, list_done, list_undone, reset, rm};
use sql::create_table;


mod args;
mod implementation;
mod sql;

fn main(){
    let args = Args::parse();

    // Create the table if it doesn't exist
    let _ = create_table();

    let _ = match args.command {
        Commands::Add { items } => {
            let _ = add(items);
        }, 
        Commands::Edit { index, items } => {
            let _ = edit(index, &items);
        },
        Commands::List => {
            let _ = list();
        },
        Commands::ListDone => {
            let _ = list_done();
        },
        Commands::ListUnDone => {
            let _ = list_undone();
        },
        Commands::Done { indexes } => {
            let _ = done(indexes);
        },
        Commands::Rm { index } => {
            let _ = rm(index);
        },
        Commands::Reset => {
            let _ = reset();
        },
        Commands::Sort => {
            let _ = list();
        },
    };
}
