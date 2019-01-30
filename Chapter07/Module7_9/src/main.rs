fn main() {
    mod instrument{
        pub fn clarinet() {
            println!("clarinet begin");
            super::breathe_in();
            println!("clarinet end");
        }
    }
    instrument::clarinet();
    println!("Hello, world!");
}

fn breathe_in() {
    // Function body code goes here
    println!("breathe in");
}
