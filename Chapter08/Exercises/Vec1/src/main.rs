fn main() {
    let mut v = Vec::new();
    let mut total = 0;

    v.push(5);
    v.push(8);
    v.push(6);
    v.push(7);
    v.push(3);
    v.push(4);
    for i in &v {
        total += i;
    }
    let avg = total / v.len();
    println!("avg is {}", avg);
    v.sort();
    let mid = v.len()/2;
    println!("mid is {}", mid);
    for i in &v {
        println!("{}", i);
    }

    println!("Hello, world!");
}
