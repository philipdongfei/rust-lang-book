
fn main() {
    use std::io;
    use std::fs;

    fn read_username_from_file() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }

    let f = read_username_from_file();

    let f = match f{
        Ok(String) => String,
        Err(error) => panic!("There is a error of open file"),
    };
}
