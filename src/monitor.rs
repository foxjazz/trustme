// #![feature(nll)]
// use x11;


// use clipboard;

mod tio;
use tio::GData;

pub fn start() {
    
        let mut myList = GData::new();
        // let mut myListD = &mut myList;
        loop {
            tio::out("tm>");
            let line = tio::readln();
            match tio::readln().as_str() {
                "add" => addNameSecret(&mut myList),
                "list" => myList.list(),
                "quit" | "exit" => return,
                _ => return,
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
}

fn setToClipboard() {
    //  let mut ctx = clipboard::ClipboardContext::new().unwrap();
    // ctx.set_contents("some string".to_owned()).unwrap();
}