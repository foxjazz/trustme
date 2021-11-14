
// use x11;


// use clipboard;
use std::collections::HashMap;
mod tio;
use tio::GData;

pub fn start() {
    
    let mut myList = GData::new();

    tio::out(">");
    let line = tio::readln();
    if line == "add" {
        addNameSecret(&mut myList);
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
}

fn setToClipboard() {
    //  let mut ctx = clipboard::ClipboardContext::new().unwrap();
    // ctx.set_contents("some string".to_owned()).unwrap();
}