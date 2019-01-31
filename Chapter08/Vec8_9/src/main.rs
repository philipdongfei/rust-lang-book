fn main() {
    let mut v = vec![10,32,57];
    for i in &mut v {
        *i += 50;
    }
    for i in &mut v{
        println!("{}", i);
    }
    println!("Hello, world!");
}
