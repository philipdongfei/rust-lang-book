fn main() {
    let s = String::from("hello world");

    //let hello = &s[0..5]; // &s[0..=4];
    let hello = &s[..5];
    println!("{}", hello);
    //let world = &s[6..11];//&s[6..=10];
    let world = &s[6..];
    println!("{}", world);
    
    println!("{}", s);
    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }
}
