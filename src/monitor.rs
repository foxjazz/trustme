// #![feature(nll)]
// use x11;


// use clipboard;
use std::env;
use std::env::join_paths;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use serde_json;
mod tio;
use tio::GData;
pub fn setup(){
    let mut current =  env::current_dir().unwrap();

    current = current.join("/data.tm");
    if (Path::new(&current).exists()) {
        let contents: String = std::fs::read_to_string(current).unwrap();
        if contents.len() > 0 {
            let mut data_v: GData = serde_json::from_str(contents.as_str()).unwrap();
//        let &mut data = data_v;
            start(data_v);
        }
    }
    else  {
        let mut data = GData::new();
        start(data);
    }
    // let bytes: Vec<u8> = std::fs::read("/some/file")?;

}
pub fn saveJson(myList: &GData){
    let mut current =  env::current_dir().unwrap();
    current = current.join("/data.tm");
    let answer = serde_json::to_string(myList)
        .expect("error");
    let mut f = File::create(current)
        .expect("error");
    f.write_all(answer.as_bytes()).expect("TODO: panic message");
}
pub fn start(mut myList: GData) {

        // let mut myListD = &mut myList;
        loop {
            tio::out("tm>");
            let s = tio::readln();
            let mut iterator = s.split(" ");
            let mut resolved = false;
            while let Some(word) = iterator.next() {
                match word {
                    "add" => {
                        if let Some(second) = iterator.next()
                        {
                            if let Some(third) = iterator.next() {
                                let mut data = (second.to_string(), third.to_string());
                                myList.add(data);
                                resolved = true;
                                break;
                            } else {
                                break;
                            }
                        } else{
                            break;
                        }
                    },
                    "get" => {
                        if let Some(second) = iterator.next() {
                            myList.get(second.to_string());
                        } else {
                            break;
                        }
                    }
                    _ => continue,
                }
            }
            if resolved == true{

                continue;
           }
            else {
                match s.as_str() {
                    "add" => addNameSecret(&mut myList),
                    "list" => myList.list(),
                    "get" => {
                        tio::out("\r\nget:");
                        myList.get(tio::readln());
                    },
                    "help" => {
                        tio::out_ln("commands: add, list, get, quit");
                    },
                    "quit" | "exit" => return,
                    _ => return,
                }
            }

    }

}

/* fn checkKeys(){
    let mut key;
    x11::XGrabKey(key)
} */
fn addNameSecret(myList: &mut GData) {
    tio::out("name:");
    let line = tio::readln();
    tio::out("pass:");
    let pass = tio::readln();
    let mut data = (line, pass);
    myList.add(data);
    saveJson(&myList);
}

fn setToClipboard() {
    //  let mut ctx = clipboard::ClipboardContext::new().unwrap();
    // ctx.set_contents("some string".to_owned()).unwrap();
}