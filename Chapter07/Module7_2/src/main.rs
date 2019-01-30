mod sound {
    mod instrument {
        mod woodwind{
            fn clarinet() {
                // Function body code goes here
                println!("clarinet");
            }
        }
    }
    mod voice {

    }
}
fn main() {
    // Absolute path
    crate::sound::instrument::clarinet();

    // Relative path
    sound::instrument::clarinet();
    println!("Hello, world!");
}
