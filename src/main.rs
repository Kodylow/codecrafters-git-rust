use flate2::read::ZlibDecoder;
use serde_derive::Deserialize;
use std::env;
use std::fs;
use std::io::Read;
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
    info!("Initialized git directory")
}

fn git_cat_file(args: &Vec<String>) {
    let hash = &args[3];
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

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    info!("Logs from your program will appear here!");

    let mut args: Vec<String> = env::args().skip(1).collect();
    if let Some(command) = args.get(0).cloned() {
        args.remove(0); // Remove the command from the args list
        match command.as_str() {
            "init" => git_init(),
            "cat-file" => git_cat_file(&args),
            _ => println!("unknown command: {}", command),
        }
    }
}
