use super::schema::shower_data;
use super::*;
use diesel::SqliteConnection;
use serde::{Serialize, Deserialize};


#[derive(Queryable, Insertable, Debug, Serialize, Deserialize, Clone)]
#[table_name="shower_data"]
pub struct ShowerData {
    pub start_unix_date: i64,
    pub duration: i32,
}

pub fn insert_data(conn: &SqliteConnection, start_unix_date: i64, duration: i32) {

    let new_post = ShowerData {
        start_unix_date,
        duration
    };
    while let Err(e) = diesel::insert_into(shower_data::table)
        .values(&new_post)
        .execute(conn){
        println!("Error inserting {}", e.to_string())
    }
}


pub fn read_all_data(conn: &SqliteConnection) -> Vec<ShowerData>{
    loop {
        match shower_data::table.load(&*conn) {
            Ok(data) => {
                return data;
            },
            Err(e) => {
                println!("Error reading: {}", e.to_string());
            }
        }
    }
}