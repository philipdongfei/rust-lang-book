fn main() {
    println!("Hello, world!");
    let hello = String::from("السلام عليكم");
    println!("{}", hello);
    let hello = String::from("Dobr den");
    println!("{}", hello);
    let hello = String::from("Hello");
    println!("{}", hello);
    let hello = String::from("שָׁלוֹם");
    println!("{}", hello);
    let hello = String::from("नमस्ते");
    for c in hello.chars() {
        println!("{}", c);
    }
    for b in hello.bytes() {
        println!("{}", b);

    }
    println!("{}", hello);
    let hello = String::from("こんにちは");
    println!("{}", hello);
    let hello = String::from("안녕하세요");
    println!("{}", hello);
    let hello = String::from("你好");
    println!("{}", hello);
    let hello = String::from("Olá");
    println!("{}", hello);
    let hello = String::from("Здравствуйте");
    let s = &hello[0..4];
    println!("s is {}", s);
    println!("{}", hello);
    let hello = String::from("Hola");
    println!("{}", hello);
    let len = String::from("Hola").len();
    println!("hello len = {}", len);
    let len = String::from("Здравствуйте").len();
    println!("Cyrillic letter len = {}", len);
}
