use godsays::God;

fn main() {
    let god = God::init("Happy.TXT", 32);
    println!("{}", god.speak());
}
