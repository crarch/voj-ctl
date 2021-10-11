
use std::fs::File;

use bson;
use mongodb::bson::doc;
use bson::document::Document;
use std::io::{self,BufRead};

use crate::get_env;
use crate::get_mongo_database;

pub fn add_user(user_data_file:&str){
    
    let users_data=File::open(user_data_file).unwrap();
    
    
    let database=get_mongo_database();
    let collection=database.collection::<Document>("users");

    let salt=get_env("HASH_SALT")   ;
    
    
    let mut lines=io::BufReader::new(users_data).lines();
    
    while let Some(line)=lines.next(){
        if let Ok(line)=line{
            let line_v:Vec<&str>=line.split(' ').collect();
            
            let user_id=line_v[0].parse::<u32>().unwrap();
            
            let user_email=line_v[1].to_string();
            let user_password=line_v[2];
            
            let config=argon2::Config::default();
            
            let hash=argon2::hash_encoded(user_password.as_bytes(),salt.as_bytes(),&config).unwrap();
            
            let _pass:Vec<u32>=Vec::new();
            
            let new_user=doc!{
                "user_id":user_id,
                "user_email":user_email,
                "user_password":hash,
            };
            
            
            collection.update_one(
                doc!{"user_id":user_id},
                doc!{
                    "$set":new_user
                },
                mongodb::options::UpdateOptions::builder()
                    .upsert(true)
                    .build()
            ).unwrap();
            
        }
    }
    
}
