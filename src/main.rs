use std::io;
use std::io::ErrorKind;

fn main() {
    let mut buffer = String::with_capacity(4096);
    println!("string buffer allocated");

    println!("please type the directory to the file to load into heap memory (e.g., \"C:\\Users\\<user>\\Documents\\file.txt\")");

    match io::stdin().read_line(&mut buffer) {
        Ok(bytes_read) => {
            println!("read {bytes_read} bytes")
        }
        Err(error) => {
            println!("failed to read line from console");
            match error.kind() {
                ErrorKind::InvalidData => {
                    
                }

                _ => {

                }
            }
        }
    }
}