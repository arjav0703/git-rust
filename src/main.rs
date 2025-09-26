use anyhow::Context;
use flate2::read::ZlibDecoder;
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
}
