use std::io::Write;

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::File;


struct mydata
{
    name: String;
    val: String;
}
impl mydata {
    fn new (name: &str, val: &str) -> {
        mydata {name: name.to_string(), val: val.to_string()
        }
    }
}

pub fn out(data: &str){
    std::io::stdout().write(data.as_bytes()).unwrap();
    std::io::stdout().flush();
}

pub fn readln() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.pop();
    line.pop();
    return line;
}
pub fn md(dir: &str){

    std::fs::create_dir(dir);

}

pub fn save(){
    let mut ser = serde_json::to_string(&mydata).unwrap();
    let mut file = File::create("mydata.tm").unwrap();
    file.write_all(ser).unwrap();

}
pub fn addData(one: &str, two: &str){
    mydata.insert(one, two);
}
/* pub fn read() -> :HashMap {

} */