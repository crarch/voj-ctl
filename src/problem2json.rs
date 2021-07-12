use std::fs;
use std::fs::File;
use std::path::Path;
use bson;
use mongodb::bson::doc;
use bson::document::Document;
use std::io::{self,BufRead};

use serde_json::json;
use serde_json::map::Map;
use serde::{Deserialize,Serialize};

use crate::get_unix_timestamp_mills;
#[derive(Debug,Serialize,Deserialize)]
struct ProblemDigest{
    tid:u32,
    order:u32,
    title:String,
}

#[derive(Debug,Serialize,Deserialize)]
struct ProblemSets{
    name:String,
    problems:Vec<ProblemDigest>,
}

#[derive(Debug,Serialize,Deserialize)]
struct ProblemList{
    updated_at:u64,
    problem_set:Vec<ProblemSets>,
}

use std::process::Command;
pub fn problem2json(problem_folder:&str,target_folder:&str){
    
    //cmd mkdir -p target_folder
    
    let mut list=ProblemList{
        updated_at:get_unix_timestamp_mills(),
        problem_set:Vec::new()
    };
    
    let mut mkdir=Command::new("mkdir");
    mkdir.arg("-p");
    mkdir.arg(target_folder);
    let _=mkdir.output();
    
    let root=Path::new(problem_folder);
    let types=root.read_dir().unwrap(); 
    

    
    for typ in types{
    
        let typ=typ.unwrap();
        
        let typ_name=typ.path().clone().file_name().unwrap().to_owned().into_string().unwrap();
        
        let typ_path_str=typ.path().clone().into_os_string().into_string().unwrap();
        let typ_root=Path::new(&typ_path_str);    
        let problems=typ_root.read_dir().unwrap();
        
        let mut problem_set=ProblemSets{
            name:typ_name,
            problems:Vec::new()
        };

        for problem_ids in problems{
            
                

            let problem_id=problem_ids.unwrap();
            let problem_id_string=problem_id.path().clone().file_name().unwrap().to_owned().into_string().unwrap();
            let problem_id_int:u32=problem_id_string.parse::<u32>().unwrap();


            let problem_id_path=problem_id.path().read_dir().unwrap();
            for file in problem_id_path{
                let target_folder=target_folder.to_string()+"/"+&problem_id_string;
                
                let file_path=file.unwrap().path();
                let file_name_string=file_path.clone().file_name().unwrap().to_owned().into_string().unwrap();
                let file_path_string=file_path.clone().into_os_string().into_string().unwrap();
                
                
                let mut title="".to_string();
                let mut order:u32=114514;
                let mut tid:u32=114514;
                
                
                
                if(file_name_string=="problem.md"){
                    let contents = fs::read_to_string(&file_path_string).unwrap();
                    
                    let mut lines=contents.lines();
                    
                    let mut contents = Vec::new();
                    
                    while let Some(line)=lines.next(){
                            if(line.starts_with("---")){
                                break;
                            }
                    }
                    
                    let mut problem_json=Map::new();
                    
                    while let Some(line)=lines.next(){
                        if(line.starts_with("---")){
                            break;
                        }else{
                            let line_v:Vec<&str>=line.split(':').collect();
                            if(line_v[0].starts_with("tid")){
                                
                                tid=line_v[1].to_string().parse::<u32>().unwrap();
                                let _=problem_json.insert(
                                    "tid".to_string(),
                                    serde_json::Value::from(tid)
                                );
                            }else if(line_v[0].starts_with("order")){
                                order=line_v[1].to_string().parse::<u32>().unwrap();
                                let _=problem_json.insert(
                                    line_v[0].to_string(),
                                    serde_json::Value::from(order)
                                );
                            }else if(line_v[0].starts_with("title")){
                                title=line_v[1].to_string();
                                let _=problem_json.insert(
                                    line_v[0].to_string(),
                                    serde_json::Value::from(title.clone())
                                );
                            }else{
                                let _=problem_json.insert(
                                    line_v[0].to_string(),
                                    serde_json::Value::String(line_v[1].to_string())
                                );
                            }
                        }
                            
                    }
                    
                    let problem_digest=ProblemDigest{
                        title:title,
                        order:order,
                        tid:tid
                    };
                    
                    problem_set.problems.push(problem_digest);
                    
                    while let Some(line)=lines.next(){
                        contents.push(line);
                        contents.push("\n");
                    }
                    
                    let contents:String=contents.into_iter().collect();

                            
                    let _=problem_json.insert(
                        "content".to_string(),
                        serde_json::Value::String(contents)
                    );
                    
                    //cmd mkdir -p target_folder
                    
                    let mut mkdir=Command::new("mkdir");
                    mkdir.arg("-p");
                    mkdir.arg(&target_folder);
                    let _=mkdir.output();
                    
                    let problem_path_string=target_folder+"/problem.json";
                    
                    serde_json::to_writer(&File::create(&problem_path_string).unwrap(), &problem_json).unwrap();
                
                }else{
                    //cp file target/file
                    let mut cp=Command::new("cp");
                    cp.arg(&file_path_string);
                    cp.arg(&(target_folder+"/"+&file_name_string));
                    cp.output().unwrap();
                    
                }    
                
            }
        }
        
        problem_set.problems.sort_by_key(|x|x.order);
        
        list.problem_set.push(problem_set);
    }
    
    
    
    

    let list_path=target_folder.to_string()+"/list.json";
    
    serde_json::to_writer(&File::create(&list_path).unwrap(), &list).unwrap();
    
}
