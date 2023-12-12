use std::collections::HashMap;
use std::path::{PathBuf, Path};
use sha2::{Sha256, Digest};
use std::fs::{File, self};
use std::io::{BufReader, Read};

use clap::Parser;

/// Utility that generates a SHA1 hash of a file.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {

    /// The file to hash.
    dir_name: PathBuf,
}

/// The main function is the entry point of the program.
fn main() {
    let cli = Cli::parse();

    let blacklist = [".git", "node_modules", "target", ".github", ".gitkeep", "Cargo.lock", ".DS_Store", ".gitlab", "dist", "bin", "build"];

    let h = traverse_dir(&cli.dir_name, &blacklist);
    println!("{:#?}", h);
}

/// Generates a SHA256 hash of a file.
fn hash(path: &Path) -> std::io::Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0; 65536];
    let mut hasher = Sha256::new();

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }

    let result = hasher.finalize();


    Ok(hex::encode(result).to_uppercase())
}

/// Traverses a directory and generates a SHA256 hash of each file.
/// The hash is stored in a map, where the key is the hash,
/// and the value is a vector of all the paths that have the same hash.
/// The map is returned as a result.
/// Filter out the following files and directories
/// - .git
/// - node_modules
/// - target
/// - .github
/// - LICENSE
/// - Cargo.toml
/// - README.md
/// - .gitkeep
/// - .DS_Store
/// - .gitlab-ci.yml
/// - .gitlab
fn traverse_dir(path: &Path, blacklist: &[&str]) -> std::io::Result<HashMap<String, Vec<PathBuf>>> {
    let mut map = HashMap::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if  blacklist.into_iter().any(|&s| path.ends_with(s)) {
            continue;
        }

        if path.is_dir() {
            let m = traverse_dir(&path, blacklist)?;
            map.extend(m);

        } else {
            let hash = hash(&path)?;
            let v = map.entry(hash).or_insert(Vec::new());
            v.push(path);
        }
    }

    Ok(map)
}
