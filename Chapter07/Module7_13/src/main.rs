mod sound {
    pub mod instrument {
        pub fn clarinet() {
            //Function body code goes here
            println!("clarinet");
        }
    }
}

use crate::sound::instrument;

fn main() {
    instrument::clarinet();
    instrument::clarinet();
    instrument::clarinet();
    println!("Hello, world!");
}
