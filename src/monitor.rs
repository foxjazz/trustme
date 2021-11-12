
// use x11;
use clipboard;

pub fn start() {

    let mut line = String::new();
    println!(">>");
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    if line == "add" {
        addNameSecret();
    }

}

/* fn checkKeys(){
    let mut key;
    x11::XGrabKey(key)
} */


fn addNameSecret() {
    let mut line = String::new();
    let mut pass = String::new();
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    println!("pass:");
    let b1 = std::io::stdin().read_line(&mut pass).unwrap();

}

fn setToClipboard() {
    //  let mut ctx = clipboard::ClipboardContext::new().unwrap();
    // ctx.set_contents("some string".to_owned()).unwrap();
}