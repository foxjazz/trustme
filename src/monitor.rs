
// use x11;


// use clipboard;

mod tio;

pub fn start() {

    tio::out(">");
    let line = tio::readln();
    if line == "add" {
        addNameSecret();
    }


}

/* fn checkKeys(){
    let mut key;
    x11::XGrabKey(key)
} */


fn addNameSecret() {
    tio::out("name:");
    let line = tio::readln();
    tio::out("pass:");
    let pass = tio::readln();

}

fn setToClipboard() {
    //  let mut ctx = clipboard::ClipboardContext::new().unwrap();
    // ctx.set_contents("some string".to_owned()).unwrap();
}