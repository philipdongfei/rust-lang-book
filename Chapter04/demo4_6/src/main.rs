fn main() {
    let mut s = String::from("hello");

    change(&mut s);
    println!("The value of s: {}", s);

    /*  error. cannot borrow variable as mutable more than once at a time.
     * Rust can prevent data races at compile time.
    let r1 = &mut s;
    let r2 = &mut s;

    println!("{},{}", r1, r2);
    */
    {
        let r1 = &mut s;
    }// r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    let rr1 = &s; // no problem
    let rr2 = &s; // no problem
    let rr3 = &mut s; // big problem
    println!("{},{},{}", rr1, rr2, rr3);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
