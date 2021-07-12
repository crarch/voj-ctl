mod utils;
use utils::*;
use std::io::prelude::*;

mod add_testbench;
mod add_user;
mod problem2json;

use add_testbench::add_testbench;
use add_user::add_user;
use problem2json::problem2json;

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


fn main(){
        
    let args:Vec<String>=std::env::args().collect();
    
    
    if(args[1]=="u"){
        add_user(&args[2]);
    }else if(args[1]=="t"){
        add_testbench(&args[2]);
    }else{
        problem2json(&args[2],&args[3]);
    }
            
    
}



