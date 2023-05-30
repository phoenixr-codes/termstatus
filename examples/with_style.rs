extern crate termstatus;
use termstatus::TermStatus;

#[allow(dead_code)]
#[derive(TermStatus)]
enum Status {
    #[style(yellow)]
    Building,
    Built,
    #[style(red, italic, on_yellow)]
    Finished,
}

fn main() {
    println!("{} foo", Status::Building);
    println!("{} foo", Status::Built);
    println!("{} bar", Status::Finished);
}
