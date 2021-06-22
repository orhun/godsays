use godsays::God;

fn main() {
    let god = God::init("HAPPY.txt", 32);
    println!("{}", god.speak());
}
