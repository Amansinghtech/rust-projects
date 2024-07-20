use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

fn main() {
//     get args with src and output
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: file_encryptor <src> <output>");
        return;
    }

    let src = &args[1];
    let output = &args[2];

    let content = read_file(src);

    let encrypted_content = encrypt(&content);

    write_to_file(output, &encrypted_content);
}

fn write_to_file(output: &String, encrypted_content: &Vec<u8>) {
    let mut writable_file = File::create(output);

    match writable_file {
        Ok(mut file) => {
            file.write_all(&encrypted_content).expect("Failed to write to file");
        },
        Err(error) => panic!("Problem creating the data file: {:?}", error),
    }
    println!("File encrypted successfully");
}

fn encrypt(input: &Vec<u8>) -> Vec<u8> {
    return input.iter().map(|byte| byte ^ 0b10101010).collect(); // xor with 0b10101010
}

fn read_file(src: &String) -> Vec<u8> {
    //     check if source file exists
    let path = Path::new(src);
    if !path.exists() {
        panic!("File `{}` does not exist", src)
    }

    // read file content in bytes
    let mut readable_file = File::open(src);

    let mut file = match readable_file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the data file: {:?}", error),
    };

    // read file content in bytes
    let mut file_content = Vec::new();
    file.read_to_end(&mut file_content).unwrap();

    file_content
}
