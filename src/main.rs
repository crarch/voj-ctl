use std::fs;
use std::fs::File;
use std::path::Path;
use bson;
use mongodb::bson::doc;
use bson::document::Document;
use std::io::{self,BufRead};
fn main(){
        
    
    let args:Vec<String>=std::env::args().collect();
    
    if(args[1]=="q"){
        add_question(&args[2]);
    }else if(args[2]=="u"){
        add_user(&args[2]);
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


fn add_user(user_data_file:&str){
    
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
            
            let pass:Vec<u32>=Vec::new();
            
            let new_user=doc!{
                "_id":user_id,
                "user_email":user_email,
                "user_password":hash,
                "pass":pass,
            };
            
            
            collection.update_one(
                doc!{"_id":user_id},
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

fn add_question(question_folder:&str){
    let path1=Path::new(question_folder);    
    let questions=path1.read_dir().unwrap();

    let database=get_mongo_database();
    let collection=database.collection::<Document>("testbenches");


    for question_ids in questions{

        let question_id=question_ids.unwrap();
        let question_id_int=question_id.path().clone().file_name().unwrap().to_owned().into_string().unwrap();
        if(question_id_int.starts_with(".git")){
            continue;
        }
        let question_id_int:u32=question_id_int.parse::<u32>().unwrap();

        let mut test_bench:Document=Document::new();

        let question_id=question_id.path().read_dir().unwrap();
        for test_bench_file in question_id{
            let test_bench_path=test_bench_file.unwrap().path();
            let test_bench_name=test_bench_path.clone().file_name().unwrap().to_owned().into_string().unwrap();
            let len=test_bench_name.len();
            let test_bench_name=&(test_bench_name)[..len-2].to_string();
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
