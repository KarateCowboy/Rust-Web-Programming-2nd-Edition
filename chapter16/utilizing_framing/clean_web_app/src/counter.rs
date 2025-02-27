extern crate serde_pickle;

use serde::{Deserialize, Serialize};
use std::env;


#[derive(Serialize, Deserialize, Debug)]
pub struct Counter {
    pub count: i32
}


impl Counter {

    fn get_redis_url() -> String {
        let redis_url = env::var("REDIS_URL").unwrap();
        return redis_url 
    }

    pub fn save(self) {
        let serialized = serde_pickle::to_vec(&self, Default::default()).unwrap();
        let client = redis::Client::open(Counter::get_redis_url()).unwrap();
        let mut con = client.get_connection().unwrap();

        let _ : () = redis::cmd("SET").arg("COUNTER").arg(serialized).query(&mut con).unwrap();
    }

    pub fn load() -> Counter {
        let client = redis::Client::open(Counter::get_redis_url()).unwrap();
        let mut con = client.get_connection().unwrap();
        let byte_data: Vec<u8> = redis::cmd("GET").arg("COUNTER").query(&mut con).unwrap();

        let deserialized: Counter = serde_pickle::from_slice(&byte_data, Default::default()).unwrap();
        return deserialized
    }
}