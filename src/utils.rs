use std::time::{SystemTime, UNIX_EPOCH};
pub fn get_unix_timestamp()->u32{
    let result=SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as u32;
    
    result
        
}

pub fn get_unix_timestamp_mills()->u64{
    let result=SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64;
    
    result
        
}

use dotenv;

pub fn get_env(env_name:&str)->String{
    dotenv::dotenv().ok();
    std::env::var(env_name).expect(&(format!("missing environment variable {}",env_name))[..])
}

use mongodb::sync::Client;


pub fn get_mongo_database()->mongodb::sync::Database{
    
    let mongo_url=get_env("MONGODB_URL");
    let mongo_dbname=get_env("MONGODB_DBNAME");
    
    let client=Client::with_uri_str(&mongo_url).unwrap();
    let database=client.database(&mongo_dbname);
    
    database
}
