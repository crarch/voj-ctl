use std::fs;

use std::path::Path;
use bson;
use mongodb::bson::doc;
use bson::document::Document;



use crate::get_mongo_database;

use crate::get_unix_timestamp;

pub fn add_testbench(question_folder:&str){
    let path1=Path::new(question_folder);    
    let questions=path1.read_dir().unwrap();

    let database=get_mongo_database();
    let collection=database.collection::<Document>("testbenches");

    for question_ids in questions{

        let question_id=question_ids.unwrap();
        let question_id_int=question_id.path().clone().file_name().unwrap().to_owned().into_string().unwrap();
        if question_id_int.starts_with(".git") {
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
