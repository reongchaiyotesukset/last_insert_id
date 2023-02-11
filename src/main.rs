use std::rc::Rc;
use mysql::*;
use gtk::prelude::*;
use mysql::prelude::*;
fn main() -> std::result::Result<(), Box<dyn std::error::Error>> 
{
    let pool = Pool::new("mysql://root:12345678@localhost:3306/testdb").unwrap();
    let mut conn = pool.get_conn().unwrap();

    //conn.exec_drop("CREATE TABLE IF NOT EXISTS people (id INT PRIMARY KEY AUTO_INCREMENT, name VARCHAR(255))",()).unwrap();

    //conn.exec_drop("INSERT INTO people (name) VALUES ('John Doe')",()).unwrap();
     conn.exec_drop("INSERT INTO example (data) VALUES ('John Doe')",()).unwrap();
     
    let last_id : Option<i32>  = conn
        .exec_first("SELECT LAST_INSERT_ID()", ())
        .unwrap()
        .unwrap();

    println!("The last inserted ID is: {:?}", last_id );
    
    
    Ok(())
}
