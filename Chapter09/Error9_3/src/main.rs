use std::fs::File;
use std::io::ErrorKind;

fn main() {
    /*
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    */
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", error),
        },
    };
}
