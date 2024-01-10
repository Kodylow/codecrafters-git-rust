use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use serde_derive::Deserialize;
use sha1::{Digest, Sha1};
use std::env;
use std::fs;
use std::io::Read;
use std::io::Write;
#[allow(unused_imports)]
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Debug, Deserialize)]
#[serde(tag = "command", rename_all = "kebab-case")]
enum GitArgs {
    Init,
    CatFile,
}

fn git_init() {
    fs::create_dir(".git").unwrap();
    fs::create_dir(".git/objects").unwrap();
    fs::create_dir(".git/refs").unwrap();
    fs::write(".git/HEAD", "ref: refs/heads/master\n").unwrap();
    // info!("Initialized git directory")
}

fn git_cat_file(args: &Vec<String>) {
    let hash = &args[1];
    let path = format!(".git/objects/{}/{}", &hash[..2], &hash[2..]);
    let bytes = fs::read(path).unwrap();
    // decompress
    let mut decoder = ZlibDecoder::new(&bytes[..]);
    let mut s = String::new();
    decoder.read_to_string(&mut s).unwrap();
    // split header and blob
    let blob_string = s.splitn(2, '\0').collect::<Vec<&str>>()[1];
    print!("{}", blob_string);
}

fn zlib_compress(data: &[u8]) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).unwrap();
    encoder.finish().unwrap()
}

fn git_hash_object(args: &Vec<String>) {
    let data = &args[0];
    let header = format!("blob {}\0", data.len());
    let store = format!("{}{}", header, data);
    let mut hasher = Sha1::new();
    hasher.update(store.as_bytes());
    let hash = format!("{:x}", hasher.finalize());
    let path = format!(".git/objects/{}/{}", &hash[..2], &hash[2..]);
    fs::create_dir_all(format!(".git/objects/{}", &hash[..2])).unwrap();
    let compressed_data = zlib_compress(store.as_bytes());
    fs::write(path, compressed_data).unwrap();
    println!("{}", hash);
}

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_ansi(false) // Disable ANSI escape codes
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    // info!("Logs from your program will appear here!");

    let mut args: Vec<String> = env::args().skip(1).collect();
    if let Some(command) = args.get(0).cloned() {
        args.remove(0); // Remove the command from the args list
        match command.as_str() {
            "init" => git_init(),
            "cat-file" => git_cat_file(&args),
            "hash-object" => git_hash_object(&args),
            _ => println!("unknown command: {}", command),
        }
    }
}
