fn main() {
    //let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y , z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("The value of one is: {}", one);

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May","June", "July","August", "September", "October", "November", "December"];

    let first = months[0];
    let second = months[1];
    let index = 10;
    let elemtnt = a[index];
    println!("The value of first is: {}", first);
    println!("The value of element is: {}", element);

}
