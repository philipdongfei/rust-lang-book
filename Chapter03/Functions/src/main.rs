fn five() -> i32 {
    5
}

fn main() {
    println!("Hello, world!");
    
    another_function(5, 6);

    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    let z = five();

    println!("The value of z is: {}", z);

    let u = plus_one(5);
    println!("The value of u is: {}", u);


}
fn plus_one(x: i32) -> i32{
    x + 1
}

fn another_function(x: i32, y: i32) {
    println!("Another function.");
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
