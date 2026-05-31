use std::fs::File;
use std::{fs, io};
use std::env::current_dir;
use std::io::{BufReader, Read};
use std::io::ErrorKind;
use std::path::Path;

fn main() {
    let mut buffer = String::with_capacity(4096);

    input(
        &mut buffer,
        format!("Path to the file to load into memory (either absolute or reference to {})", current_dir().unwrap().display()).as_str()
    );

    let chosen_directory = buffer.trim();

    let file = match File::open(Path::new(chosen_directory)) {
        Ok(file) => {
            file
        }
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => {
                    eprintln!("Path does not exist.");
                }
                ErrorKind::PermissionDenied => {
                    eprintln!("Path not accessible to program.");
                }
                _ => {
                    println!("Unknown error {error}");
                }
            }

            return;
        }
    };

    let file_metadata = fs::metadata(Path::new(chosen_directory)).expect("Could not get file metadata");
    let size = (file_metadata.len() + 32) as usize;

    println!("allocated {size} bytes for chosen file");

    let mut file_contents = Vec::<u8>::with_capacity(size);
    {
        let mut reader = BufReader::new(file);

        reader.read_to_end(&mut file_contents).unwrap();
    }

    println!("allocated memory, press enter to exit");

    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();
}

fn input(buf: &mut String, message: &str) {
    print!("{}", message);
    match io::stdin().read_line(buf) {
        Ok(bytes_read) => {
            println!("read {bytes_read} bytes from user input")
        }
        Err(error) => {
            println!("failed to read line from console");
            match error.kind() {
                ErrorKind::InvalidData => {

                }

                _ => {
                    println!("Unknown error: {:?}", error);
                }
            }
        }
    }
}