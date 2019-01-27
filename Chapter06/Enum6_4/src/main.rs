fn main() {
    #[derive(Debug)] // so we can inspect the state in a mimute
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }
    value_in_cents(Coin::Quarter(UsState::Alaska));

    let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    /*
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    */

    println!("Hello, world!");
}
