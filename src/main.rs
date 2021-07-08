use std::fs;
use std::path::Path;
use bson;
use mongodb::bson::doc;
use bson::Bson;
use serde_json::{Result, Value};
use bson::document::Document;

fn main(){
        
    
    let args:Vec<String>=std::env::args().collect();
    let path1=Path::new(&args[1]);    
    let questions=path1.read_dir().unwrap();
    
    
    let database=get_mongo_database();
    let collection=database.collection::<Document>("questions");
    
    
    
    
    for question_ids in questions{
        
        let question_id=question_ids.unwrap();
        let question_id_int=question_id.path().clone().file_name().unwrap().to_owned().into_string().unwrap();
        let question_id_int:u32=question_id_int.parse::<u32>().unwrap();
        
        let mut test_bench:Document=Document::new();
        
        let question_id=question_id.path().read_dir().unwrap();
        for test_bench_file in question_id{
            let test_bench_path=test_bench_file.unwrap().path();
            let test_bench_name=test_bench_path.clone().file_name().unwrap().to_owned().into_string().unwrap();
            let contents=fs::read_to_string(test_bench_path).unwrap();
            test_bench.insert(test_bench_name,contents);
        }
        
        collection.update_one(
            doc!{"_id":question_id_int},
            doc!{
                "$set":
                    doc!{
                        "update":get_unix_timestamp(),
                        "test_bench":test_bench,
                    }
            },
            mongodb::options::UpdateOptions::builder()
                .upsert(true)
                .build()
        ).unwrap();
    
    }
    
    
}

use std::time::{SystemTime, UNIX_EPOCH};
pub fn get_unix_timestamp()->u32{
    let result=SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as u32;
    
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

