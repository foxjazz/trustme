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
}

fn setToClipboard() {
    //  let mut ctx = clipboard::ClipboardContext::new().unwrap();
    // ctx.set_contents("some string".to_owned()).unwrap();
}