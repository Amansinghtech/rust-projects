use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use aes_gcm::{AeadCore, AeadInPlace, Aes256Gcm, Key, KeyInit};
use aes_gcm::aead::consts::U32;
use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::aead::OsRng;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("Usage: file_decryptor <key> <enc_file> <output>");
        return;
    }

    let key = &args[1];
    let enc_file = &args[2];
    println!("Key file: {:?}", key);
    println!("Encrypted file: {:?}", enc_file);

    let key_content = read_file(key);
    // convert key content to [u8; 32]
    let key_content = Key::<Aes256Gcm>::from_slice(key_content.as_slice());

    println!("Key content: {:?}", key_content);

    let enc_file_content = read_file(enc_file);
    let decrypted_content = decryptor(key_content, enc_file_content);

    println!("Decrypted content: {:?}", decrypted_content);

}

fn decryptor(key: &GenericArray<u8, U32>, mut enc_file: Vec<u8>) -> Vec<u8> {
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    cipher.decrypt_in_place(&nonce, b"", &mut enc_file).expect("failed decrypting data");
    return enc_file;
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
