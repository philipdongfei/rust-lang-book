mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // Function body code goes here
            println!("clarinet");
        }
    }
}
fn main() {
    // Absolute path
    crate::sound::instrument::clarinet();

    // Relative path
    sound::instrument::clarinet();
    println!("Hello, world!");
}
