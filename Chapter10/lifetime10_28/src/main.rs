fn main() {
    use std::fmt::Display;

    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str where T: Display
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    let s1 = String::from("But now it has an");
    let s2 = String::from("extra parameter named an of");
    let ann = 1;
    println!("The largest is {}", longest_with_an_announcement(s1.as_str(), s2.as_str(), ann));
}
