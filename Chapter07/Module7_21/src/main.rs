mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // Function body code goes here
        }
    }
}
mod performance_group {
    pub use crate::sound::instrument;

    pub fn clarinet_trio(){
        instrument::clarinet();
        instrument::clarinet();
        instrument::clarinet();
    }
}
fn main() {
    performance_group::clarinet_trio();
    performance_group::instrument::clarinet();
    println!("Hello, world!");
}
