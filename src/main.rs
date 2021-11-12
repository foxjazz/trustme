mod monitor;
mod setup;
fn main() {
    setup::setup();
    monitor::start();
}
