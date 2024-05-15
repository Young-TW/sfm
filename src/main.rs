use std::collections::HashMap;
use std::vec::Vec;
use std::fs;
use std::env;
use std::path::Path;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a file path");
        return;
    }

    let path = Path::new(&args[1]);
    if !path.is_dir() {
        println!("Please provide a directory path");
        return;
    }

    delete_duplicates(process_dir(path));
}

fn process_dir(path: &Path) -> HashMap<String, Vec<u8>> {
    let mut map = HashMap::new();
    for entry in fs::read_dir(path).expect("Unable to read directory") {
        let entry = entry.expect("Unable to read directory entry");
        let path = entry.path();
        if path.is_dir() {
            map.extend(process_dir(&path));
        } else {
            map.insert(path.to_str().unwrap().to_string(), process_file(&path));
        }
    }

    return map;
}

fn process_file(path: &Path) -> Vec<u8> {
    let content = fs::read(path).expect("Unable to read file");

    let mut hasher = Sha256::new();
    hasher.input(&content);

    return hasher.result_str().as_bytes().to_vec();
}

fn delete_duplicates(map: HashMap<String, Vec<u8>>) {
    let mut duplicates = HashMap::new();
    for (path, hash) in map.iter() {
        if duplicates.contains_key(hash) {
            println!("Deleting duplicate file: {}", path);
            fs::remove_file(path).expect("Unable to delete file");
        } else {
            duplicates.insert(hash, path);
        }
    }
}