








mod utils;
use utils::*;

mod add_testbench;
mod add_user;

use add_testbench::add_testbench;
use add_user::add_user;

fn main(){
        
    
    let args:Vec<String>=std::env::args().collect();
    
    if args[1]=="q" {
        add_testbench(&args[2]);
    }else if args[2]=="u" {
        add_user(&args[2]);
    }
    
    
}




