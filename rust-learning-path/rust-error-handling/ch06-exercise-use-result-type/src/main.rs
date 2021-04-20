use std::fs::File;
use std::io::{Error as IoError, Read};
use std::path::PathBuf;

fn read_file_contents(path: PathBuf) -> Result<String, IoError> {
    let mut string = String::new();

    let mut file: File = match File::open(path) {
        Ok(file_handle) => file_handle,
        Err(io_error) => return Err(io_error),
    };

    match file.read_to_string(&mut string) {
        Ok(_) => (),
        //Err(io_error) => todo!()
        Err(io_error) => return Err(io_error),
    };

    Ok(string)
}


fn main() {
    match read_file_contents(PathBuf::from("src/main.rs")) {
        Ok(s) => println!("ok: {}",s),
        Err(io_error) => println!("error: {}", io_error),
    }

    assert!(read_file_contents(PathBuf::from("src/main.rs")).is_ok());
    assert!(read_file_contents(PathBuf::from("non-existent-file.txt")).is_err());
}
