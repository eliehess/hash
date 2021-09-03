use std::env;
use std::fs;
use std::io::Read;
use sha2::{Sha256, Digest};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => { handle_raw(&args[1]); },
        3 => match args[1].as_str() {
            "-r" | "--raw" => { handle_raw(&args[2]); },
            "-f" | "--file" => { handle_file(&args[2]); },
            _ => { eprintln!("Acceptable input types:\n\tRaw contents of next arg (-r, --raw)\n\tFrom file (-f, --file)"); }
        }
        _ => { eprintln!("Usage: hash <INPUT TYPE> [INPUT]"); }
    }
}    

fn handle_file(filename: &str) {
    match read_file(filename) {
        Ok(contents) => { println!("{}", hash(contents)); },
        Err(e) => { eprintln!("Error reading file: {}", e); }
    };    
}

fn handle_raw(input: &str) {
    println!("{}", hash(input));
}

fn read_file(filename: &str) -> std::io::Result<Vec<u8>> {
    let mut file = fs::File::open(filename)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn hash(input: impl AsRef<[u8]>) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    return hasher
        .finalize()
        .into_iter()
        .map(|x| if x < 16 { format!("0{:x}", x) } else { format!("{:x}", x) })
        .collect();
}