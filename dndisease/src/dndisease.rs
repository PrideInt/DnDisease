use std::thread;
use pathogen::Pathogen;
use pathogen::Bacteria;
mod pathogen;

fn main() {
    let handler = thread::spawn(|| {
        let bacteria: Bacteria = pathogen::new_bacteria();

        for _ in 0..10 {
            bacteria.attack();
        }
    });
    handler.join().unwrap();
}
