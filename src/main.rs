use anyhow::Context;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};
use std::env;
use std::fs;
use std::io::Read;
use std::io::Write;

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
    } else if args[1] == "ls-tree" {
        ls_tree(&args);
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
    let file_content = fs::read(filename).expect("failed to read file");

    let header = format!("blob {}\0", file_content.len());

    let mut store_content = Vec::new();
    store_content.extend_from_slice(header.as_bytes());
    store_content.extend_from_slice(&file_content);

    // Compute SHA1 hash of store_content
    let mut hasher = Sha1::new();
    hasher.update(&store_content);
    let object_hash = hasher.finalize();
    let object_hex = hex::encode(object_hash);

    let dir = format!(".git/objects/{}", &object_hex[0..2]);
    fs::create_dir_all(&dir).unwrap();

    let object_path = format!("{}/{}", dir, &object_hex[2..]);

    // Compress with zlib
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&store_content).unwrap();
    let compressed = encoder.finish().unwrap();

    fs::write(&object_path, compressed).expect("failed to write object");

    println!("{}", object_hex);
}

fn ls_tree(args: &[String]) {
    let (object_hash, name_only) = if args[2] == "--name-only" {
        (&args[3], true)
    } else {
        (&args[2], false)
    };

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
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).unwrap();

    // header is ascii, safe to parse as UTF-8
    let nul_pos = decompressed.iter().position(|&b| b == 0).unwrap();
    let header = std::str::from_utf8(&decompressed[..nul_pos]).unwrap();
    let object_content = &decompressed[nul_pos + 1..];

    let mut header_parts = header.split(' ');
    let object_type = header_parts.next().unwrap();
    let object_size: usize = header_parts.next().unwrap().parse().unwrap();

    if object_type != "tree" {
        panic!("Not a tree object");
    }

    let mut i = 0;
    while i < object_size {
        // mode (ascii) until space
        let space_index = i + object_content[i..].iter().position(|&b| b == b' ').unwrap();
        let mode = std::str::from_utf8(&object_content[i..space_index]).unwrap();
        i = space_index + 1;

        // filename until null
        let null_index = i + object_content[i..].iter().position(|&b| b == 0).unwrap();
        let filename = std::str::from_utf8(&object_content[i..null_index]).unwrap();
        i = null_index + 1;

        // 20-byte sha1
        let sha_bytes = &object_content[i..i + 20];
        let sha_hex = hex::encode(sha_bytes);
        i += 20;

        if name_only {
            println!("{}", filename);
            continue;
        }
        println!("{} {} {}", mode, sha_hex, filename);
    }
}
