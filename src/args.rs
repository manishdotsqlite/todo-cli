use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Commands {
        Add{items: String}, // add any number of todos
        Edit{index: i32, items: String}, // edit a todo
        List, // list all todos
        ListDone, // list all done todos
        ListUnDone, // list all undone todos
        Done{indexes: Vec<i32>}, // mark a todo as done
        Rm{index: Vec<i32>}, // remove a todo
        Reset, // reset all todos
        Sort, // sort all todos
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
        #[command(subcommand)]
        pub command: Commands,
}

