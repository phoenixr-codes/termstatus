extern crate termstatus;
use termstatus::TermStatus;

#[allow(dead_code)]
#[derive(TermStatus)]
enum Status {
    Building,
    Built,
    #[display = "Cleaning Up"]
    CleaningUp,
}

fn main() {
    println!("{} foo", Status::Building);
    println!("{} bar", Status::CleaningUp);
}
