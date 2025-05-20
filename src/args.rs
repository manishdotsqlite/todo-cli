use clap::Parser;

pub enum Commands {
        Add(String), // add any number of todos
        Edit(i32, String), // edit ith task
        List, // lists all tasks
        ListDone, // lists all done tasks
        ListUnDone, // lists all undone tasks
        Done(i32), // marks task as done
        Rm(i32), // removes a task
        Reset, // deletes all tasks
        Sort, // sorts completed and uncompleted tasks
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
        
        /// Command
        #[arg()]
        pub command: String,

        /// Argument 1
        #[arg(default_value_t = 1)] 
        pub index: i32,

        /// Argument 2
        #[arg(default_value_t = String::from(""))]
        pub tasks: String
}

