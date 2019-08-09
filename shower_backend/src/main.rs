#![feature(proc_macro_hygiene, decl_macro)]
use sysfs_gpio::{Direction, Pin, Edge, PinPoller};
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH, Instant};
mod schema;
mod model;
use model::*;
use diesel::SqliteConnection;
use std::env;
use dotenv::dotenv;
use serde::{Serialize};
use flexi_logger::opt_format;
use flexi_logger::Logger;
use log::*;
#[macro_use]
extern crate diesel;
use diesel::prelude::*;

#[macro_use]
extern crate lazy_static;
use std::sync::{RwLock, Arc};


#[macro_use] extern crate rocket;

use rocket_contrib::json::Json;

#[derive(Debug, Serialize, Clone)]
enum ValueChange {
    TurnedOn(SystemTime),
    TurnedOff
}

lazy_static!{
            static ref CURRENT_VALUE: Arc<RwLock<ValueChange>> = Arc::new(RwLock::new(ValueChange::TurnedOff));
}

impl ValueChange{
    pub fn different_variant_(&self, other: Self) -> bool{
        match (&self, other){
            (ValueChange::TurnedOn(_), ValueChange::TurnedOn(_)) => {
                true
            },
            (ValueChange::TurnedOff, ValueChange::TurnedOff) => {
                true
            },
            _ => false
        }
    }

}
struct ShowerBackend{
    shower_pin: Pin,
    shower_pin_poller: PinPoller,
    last_value: ValueChange,
}

impl ShowerBackend{
    pub fn new() -> Self{
        let shower_pin = Pin::new(18);
        shower_pin.export().expect("Could not export pin 18 :(");
        sleep(Duration::from_secs(2));
        shower_pin.set_direction(Direction::In).expect("Error setting direction in");
        sleep(Duration::from_secs(2));
        shower_pin.set_edge(Edge::BothEdges).expect("Error setting edge");
        sleep(Duration::from_secs(2));
        let shower_pin_poller = shower_pin.get_poller().expect("Error getting pin poller");
        ShowerBackend{
            shower_pin,
            shower_pin_poller,
            last_value: ValueChange::TurnedOff
        }
    }

    fn set_new_global_last_value(new: ValueChange){
        *(*CURRENT_VALUE)
            .write()
            .expect("CURRENT_VALUE lock was poisoned") = new;
    }


    pub fn get_value_change(&mut self) -> ShowerData{
        'infinite: loop {
            let mut change = self.shower_pin_poller.poll(-1)
                .expect("Error during pooling")
                .expect("Timeout should never occur");
            info!("Change happened! Value: {}", change);

            'inner: loop{
                match self.shower_pin_poller.poll(1000)
                    .expect("Error during pooling"){
                    None => {
                        // timeout happened, change is stable
                        info!("No change for 1s! Declaring old value stable ({})", change);
                        break 'inner;
                    },
                    Some(new_change) => {
                        // got new change
                        change = new_change;
                        info!("Got change before stabilizing! Value: {}", change);
                    },
                };
            }



            // Was off and now is on
            if let ValueChange::TurnedOff = self.last_value {
                if change == 1 {
                    info!("Changed, was {:#?}, changed to {:#?}", self.last_value, change);

                    let new_value = ValueChange::TurnedOn(SystemTime::now());
                    Self::set_new_global_last_value(new_value.clone());
                    self.last_value = new_value;
                }
            }
            // Was on, now is Off
            if let ValueChange::TurnedOn(turned_on_at) = self.last_value {
                if change == 0 {
                    info!("Changed, was {:#?}, changed to {:#?}", self.last_value, change);
                    let shower_data = ShowerData{
                        start_unix_date: turned_on_at.duration_since(UNIX_EPOCH)
                            .expect("Shower started before UNIX EPOCH")
                            .as_secs() as i64,
                        duration: SystemTime::now().duration_since(turned_on_at)
                            .expect("Shower started before UNIX EPOCH")
                            .as_secs() as i32
                    };
                    let new_value = ValueChange::TurnedOff;

                    self.last_value = new_value.clone();
                    Self::set_new_global_last_value(new_value.clone());
                    return shower_data;
                }
            }
            info!("No change, was {:#?}, changed to {:#?}", self.last_value, change)
        }
    }
}



pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[get("/shower_data", format = "json")]
fn user() -> Json<Vec<ShowerData>> {
    let conn = establish_connection();
    let all_data = read_all_data(&conn);
    let mut new_data = Vec::new();
    let mut last_el :Option<ShowerData> = None;
    for data in all_data{
        match last_el{
            None => {
                new_data.push(data.clone());
                last_el = Some(data);
            },
            Some(ref last) => {
                if (data.start_unix_date - last.start_unix_date) < 6*60 {
                    let last_insert = new_data.last_mut().unwrap();
                    last_insert.duration = last_insert.duration + data.duration;
                }else{
                    new_data.push(data.clone());
                    last_el = Some(data);
                }
            },
        }

    }
    Json(new_data)
}


#[get("/current_shower", format = "json")]
fn current_shower() -> Json<ValueChange> {
    let value = &*(*CURRENT_VALUE)
        .read()
        .expect("CURRENT_VALUE lock was poisoned");
    Json(value.clone())
}


fn main() {

    Logger::with_str("info")
        .format(opt_format)
        .log_to_file()
        .directory("./logs")
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));

    info!("Start!");

    use model::*;
    let default = rocket_cors::CorsOptions::default();
    let cors = default.to_cors().unwrap();
    std::thread::spawn(|| {
        let mut shower_backend = ShowerBackend::new();
        let conn = establish_connection();
        loop {
            let shower_data = shower_backend.get_value_change();
            insert_data(&conn, shower_data.start_unix_date, shower_data.duration);
            info!("{:#?}", read_all_data(&conn));
        }
    });

    rocket::ignite()
        .attach(cors)
        .mount("/", routes![user, current_shower]).launch();
}

