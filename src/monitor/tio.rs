use std::io::Write;

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::File;


pub struct GData
{
    data: HashMap<String, String>,
}
impl GData {
    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }

    pub fn add(&mut self, data: (String, String)) {
        let (k, v) = data;

        self.data.insert(k, v);
    }

    fn remove(&mut self, kk: String) {

        self.data.remove(&kk);
    }
    fn list(&mut self, kk: String) -> Vec<&String> {
        let myKeys :Vec<&String> =   self.data.keys().collect();
        // List<string> lst = new List();
        myKeys
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
}/*  */
pub fn md(dir: &str){


    std::fs::create_dir(dir);

}

pub fn save(myData: GData){
    let mut ser = serde_json::to_string("data").unwrap();
    let mut file = File::create("mydata.tm").unwrap();
    file.write_all(ser.as_bytes()).unwrap();

}

// pub fn addData(one: &str, two: &str){
//     mydata.insert(one, two);
// }
/* pub fn read() -> :HashMap {

} */