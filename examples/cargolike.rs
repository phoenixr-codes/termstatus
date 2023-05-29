//! This example imitates the build process of cargo.

extern crate termstatus;
use termstatus::TermStatus;

fn sleep(duration: u64) {
    let d = std::time::Duration::from_millis(duration);
    std::thread::sleep(d);
}

#[derive(TermStatus)]
enum Status {
    Building,
    Built,
    Compiled,
    Compiling,
    Finished,
    Running,
}

fn main() {
    let interval = 130;
    println!("{} foo", Status::Building);
    sleep(interval);
    println!("{} foo", Status::Built);
    sleep(interval);
    println!("{} bar", Status::Building);
    sleep(interval);
    println!("{} bar", Status::Built);
    sleep(interval);
    println!("{} moo", Status::Compiling);
    sleep(interval);
    println!("{} moo", Status::Compiled);
    sleep(interval);
    println!("{} moo", Status::Finished);
    sleep(interval);
    println!("{} moo", Status::Running);
}
