// #![feature(nll)]
// use x11;


// use clipboard;

mod tio;
use tio::GData;

pub fn start() {
    
        let mut myList = GData::new();
        loop {
            let mut cur = myList.clone();
            let mut copObj: GData = myList.clone();
            tio::out("tm>");
            let line = tio::readln();
            if line == "add" {
               addNameSecret(&mut cur);
            }
            if line == "list"{
                list(copObj);
            }
            if (line == "quit"){
                // save
                return;
            }
    }

}

/* fn checkKeys(){
    let mut key;
    x11::XGrabKey(key)
} */
fn list (myList: GData) {
    let list = myList.list();
    let mut done: bool = false;
    let mut i: u32 = 0;
    let l = list.len();
    if l == 0 {
        return;
    }
    loop {
        let s = list[0];
            if  l == 0 {
                break;
            }
            tio::outln(s);
            i+=1;
        
    }
}

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