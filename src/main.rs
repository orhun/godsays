use godsays::God;
use std::io;

fn main() {
    let god = God::init("HAPPY.txt", 32);
    if let Err(e) = god.speak(&mut io::stdout()) {
        eprintln!("Error: {:?}", e)
    }
}
