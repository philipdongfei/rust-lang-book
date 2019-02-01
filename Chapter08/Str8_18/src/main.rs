fn main() {
    let s1 = String::from("Hello,");
    let s2 = String::from("World!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    println!("s3 is {}", s3);
    println!("s is {}", test());
    println!("s is {}", testformat());
}

fn test() -> String {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    s
}
fn testformat() -> String {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{};", s1, s2, s3);
    s
}
