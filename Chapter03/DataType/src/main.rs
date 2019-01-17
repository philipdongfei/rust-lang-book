fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("x = {}, y = {}", x, y);

    let t = true;

    let f: bool = false; // with explicit type annotation

    println!("t={},f={}", t, f);

    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';

    println!("c = {}, z = {}, heart_eyed_cat = {}", c, z, heart_eyed_cat);
}
