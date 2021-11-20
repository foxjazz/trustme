use std::io::Write;

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::File;

#[derive(Clone)]
pub struct GData
{
    data: HashMap<String, String>,
}
impl GData {
    
    pub fn new() -> Self {
        return Self { data: HashMap::new() }
    }
    
    pub fn add(&mut self, data: (String, String)) {
        let (k, v) = data;
        self.data.insert(k, v);
    }

    pub fn remove(&mut self, kk: String) {
        self.data.remove(&kk);
    }
    pub fn get(&self, query: String){
        let q: &str = query.as_str();
        out_ln(self.data.get(q).unwrap());
    }

    
    pub fn list(&mut self) {
        for (key, val) in self.data.iter() {
            out_ln( key);
        }
    }
    pub fn find(&mut self, kk: String) {
        for (key, val) in self.data.iter() {
            if key.starts_with(&kk) {
                out_ln(key);
            }
        }
        
    }
}

pub fn out(data: &str){
    std::io::stdout().write(data.as_bytes()).unwrap();
    std::io::stdout().flush();
}
pub fn out_ln(data: &str){
    std::io::stdout().write(data.as_bytes()).unwrap();
    std::io::stdout().write("\r\n".as_bytes()).unwrap();
    std::io::stdout().flush();
}

pub fn readln() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
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