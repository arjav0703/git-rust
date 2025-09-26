use anyhow::Context;
use flate2::read::ZlibDecoder;
use sha1::{Digest, Sha1};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "init" {
        fs::create_dir(".git").unwrap();
        fs::create_dir(".git/objects").unwrap();
        fs::create_dir(".git/refs").unwrap();
        fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
        println!("Initialized git directory")
    } else if args[1] == "cat-file" && args[2] == "-p" {
        cat_file(&args);
    } else if args[1] == "hash-object" && args[2] == "-w" {
        hash_object(&args);
    } else {
        println!("Unknown command");
    }
}

fn cat_file(args: &Vec<String>) {
    let object_hash = &args[3];
    let object_path = format!(
        ".git/objects/{}/{}",
        &object_hash[0..2].to_string(),
        &object_hash[2..]
    );
    dbg!(&object_path);
    let raw_content = fs::read(object_path)
        .context("read file")
        .expect("read file");

    let mut decoder = ZlibDecoder::new(&raw_content[..]);
    let content = std::io::read_to_string(&mut decoder).unwrap();

    let parts: Vec<&str> = content.splitn(2, '\0').collect();

    print!("{}", parts[1]);
}

fn hash_object(args: &Vec<String>) {
    let filename = &args[3];
    let file_content = fs::read_to_string(filename).expect("read file");

    let header = format!("blob {}\0", file_content.len());

    let store_content = format!("{}{}", header, file_content);

    let mut hasher = Sha1::new();
    hasher.update(store_content.as_bytes());
    let object_hash = hasher.finalize();

    fs::create_dir_all(format!(
        ".git/objects/{}",
        hex::encode(&object_hash)[0..2].to_string()
    ))
    .unwrap();

    let object_path = format!(
        ".git/objects/{}/{}",
        hex::encode(&object_hash)[0..2].to_string(),
        hex::encode(&object_hash)[2..].to_string()
    );

    let compressed_data = {
        let mut e = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        std::io::copy(&mut store_content.as_bytes(), &mut e).unwrap();
        e.finish().unwrap()
    };

    fs::write(object_path, compressed_data).expect("write file");
    println!("{}", hex::encode(object_hash));
}
