fn main() {
    use std::io;
    use std::io::Read;
    use std::fs::File;

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut s = String::new();

        File::open("hello.txt")?.read_to_string(&mut s);
        Ok(s)
    }

    let f = read_username_from_file();
    let f = match f {
        Ok(String) => String,
        Err(error) => panic!("There is not exist file"),
    };
    println!("The value of f is {}", f);
}
