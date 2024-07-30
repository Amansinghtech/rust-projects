use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use aes_gcm::{AeadCore, AeadInPlace, Aes256Gcm, KeyInit};
use aes_gcm::aead::OsRng;

fn main() {
    //     get args with src and output
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: file_encryptor <src> <output> <optional: Key>");
        return;
    }

    let src = &args[1];
    let output = &args[2];
    // let key = &args[3];

    // println!("Key: {:?}", key);

    let content = read_file(src);

    let encrypted_content = encrypt(content);

    write_to_file(output, &encrypted_content);
}

fn write_to_file(output: &String, encrypted_content: &Vec<u8>) {
    let writable_file = File::create(output);

    match writable_file {
        Ok(mut file) => {
            file.write_all(&encrypted_content).expect("Failed to write to file");
        }
        Err(error) => panic!("Problem creating the data file: {:?}", error),
    }
    // println!("File encrypted successfully");
}

pub fn encrypt(mut data: Vec<u8>) -> Vec<u8> {
    let key = Aes256Gcm::generate_key(OsRng);
    println!("Key Used: {:?}", key);
    write_to_file(&String::from("secret.key"), &key.to_vec());
    println!("Key written to file secret.key");
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    cipher.encrypt_in_place(&nonce, b"", &mut data).expect("failed encrypting data"); // 96-bits; unique per message
    return data;
}

fn read_file(src: &String) -> Vec<u8> {
    //     check if source file exists
    let path = Path::new(src);
    if !path.exists() {
        panic!("File `{}` does not exist", src)
    }

    // read file content in bytes
    let readable_file = File::open(src);

    let mut file = match readable_file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the data file: {:?}", error),
    };

    // read file content in bytes
    let mut file_content = Vec::new();
    file.read_to_end(&mut file_content).unwrap();

    file_content
}
