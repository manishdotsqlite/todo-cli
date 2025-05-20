use crate::sql::execute_sql;
use crate::sql::query_data;
use console::style;
use console::Style;

pub fn add(tasks: String) -> Result<(), &'static str> {
        let mut tasks_vector: Vec<String>= tasks.split("\"").filter(|s| !s.is_empty()).map(|s| s.trim().to_string()).collect();
        tasks_vector.retain(|s| s!= "");
        
        for task in tasks_vector {
                let sql = format!("INSERT INTO todo (name, done) VALUES ({}, false)", task);
                match execute_sql(sql) {
                        Ok(_) => return Ok(()),
                        Err(_) => return Err("Error when adding todo.")
                };
        }

        Ok(())
}


pub fn edit(index: i32, newtask: &str) -> Result<(), &'static str> {
        let sql = format!("UPDATE todo SET name={} WHERE index={}", newtask, index);
        match execute_sql(sql) {
                Ok(_) => return Ok(()),
                Err(_) => return Err("Error while adding todo.")
        };
}

pub fn list() -> Result<(), &'static str> {
        let strike_through = Style::new().strikethrough();
        let sql = format!("SELECT id, name, done FROM todo");
        let todos = match query_data(&sql) {
                Ok(some) => some,
                Err(_) => return Err("Couldn't get todo items.")
        };

        for todo in todos {
                let id = todo.id;
                let name = todo.name;
                let done = todo.done;
                println!("List of tasks:");
                if done {
                        println!("{}. {}", id, strike_through.apply_to(name));
                } else {
                        println!("{}. {}", id, name);
                }
        }

        Ok(())
}


pub fn list_done() -> Result<(), &'static str> {
        let sql = format!("SELECT id, name, done FROM todo WHERE done=true");
        let done_todos = match query_data(&sql) {
                Ok(some) => some,
                Err(_) => return Err("Couldn't get done todo items.")
        };

        for todo in done_todos {
                let id = todo.id;
                let name = todo.name;
                println!("List of done tasks:");
                println!("{}. {}", id, name);
        }

        Ok(())
}


pub fn list_undone() -> Result<(), &'static str> {
        let sql = format!("SELECT id, name, done FROM todo WHERE done=false");
        let undone_todos = match query_data(&sql) {
                Ok(some) => some,
                Err(_) => return Err("Couldn't get undone todo items.")
        };

        for todo in undone_todos {
                let id = todo.id;
                let name = todo.name;
                println!("List of undone tasks:");
                println!("{}. {}", id, name);
        }

        Ok(())
}


pub fn done(indexes: Vec<i32>) -> Result<(), &'static str> {
        for index in indexes {
                let sql = format!("UPDATE todo SET done=true WHERE id={}", index);
                match execute_sql(sql) {
                        Ok(_) => return Ok(()),
                        Err(_) => return Err("Error while marking todo as done.")
                };
        }

        let _ = list();
        Ok(())
}


pub fn rm (indexes: Vec<i32>) -> Result<(), &'static str> {
        for index in indexes {
                let sql = format!("DELETE FROM todo WHERE id={}", index);
                match execute_sql(sql) {
                        Ok(_) => return Ok(()),
                        Err(_) => return Err("Error while removing todo.")
                };
        }

        let _ = list();
        Ok(())
}


pub fn reset() -> Result<(), &'static str> {
        let sql = format!("DELETE FROM todo");
        match execute_sql(sql) {
                Ok(_) => (),
                Err(_) => return Err("Error while resetting todo.")
        };

        let _ = list();
        Ok(())
}


pub fn sort() -> Result<(), &'static str> {
        let sql = format!("SELECT id, name, done FROM todo ORDER BY done DESC");
        let todos = match query_data(&sql) {
                Ok(some) => some,
                Err(_) => return Err("Couldn't get todo items.")
        };

        for todo in todos {
                let id = todo.id;
                let name = todo.name;
                let done = todo.done;
                println!("List of tasks:");
                if done {
                        println!("{}. {}", id, style(name).strikethrough());
                } else {
                        println!("{}. {}", id, name);
                }
                
        }

        Ok(())
}
