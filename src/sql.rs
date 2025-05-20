use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct Todo_Item {
        pub id: i32,
        pub name: String, 
        pub done: bool
}


pub fn create_table() -> Result<(), &'static str> {
        let conn = match Connection::open("test.db") {
                Ok(some) => some,
                Err(_) => return Err("Couldn't connect to the database.")
        };

        let sql = "CREATE TABLE IF NOT EXISTS todo (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                done BOOLEAN NOT NULL
        )";

        match conn.execute(sql, []) {
                Ok(_) => (),
                Err(_) => return Err("Couldn't create the table.")
        };

        Ok(())
}

pub fn execute_sql(sql: String) -> Result<(), &'static str>{
        let conn = match Connection::open("test.db") {
                Ok(some) => some,
                Err(_) => return Err("Couldn't connect to the database.")
        };

        match conn.execute(&sql, []) {
                Ok(_) => (),
                Err(_) => return Err("Couldn't execute the sql.")
        };
        
    Ok(())
}

pub fn query_data(sql: &str) -> Result<Vec<Todo_Item>, &'static str>{
        let conn = match Connection::open("test.db") {
                Ok(some) => some,
                Err(_) => return Err("Couldn't connect to the database.")
        };

        let mut data = match conn.prepare(sql) {
                Ok(some) => some,
                Err(_) => return Err("Couldn't return todo data.")
        };

        let todos = data.query_map([], |row| {
                Ok(Todo_Item {
                        id: match row.get(0) {
                                Ok(some) => some,
                                Err(_) => 9999
                        },
                        name: match row.get(1) {
                                Ok(some) => some,
                                Err(_) => "None".to_owned()
                        },
                        done: match row.get(2) {
                                Ok(some) => some,
                                Err(_) => false
                        },
                })
        });

        let todo_vec: Vec<Todo_Item> = match todos {
                Ok(mapped_rows) => mapped_rows.filter_map(Result::ok).collect(),
                Err(_) => return Err("Couldn't map todo data."),
        };

        Ok(todo_vec)
}
