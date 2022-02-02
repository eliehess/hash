use std::{env, fs, io::{self, Read}};
use hash::{
    sha1::sha1,
    sha2::{sha256, sha512},
    sha3::{sha3_256, sha3_512},
    md5::md5
};

macro_rules! exit {
    ($($x:expr),*) => {{
        println!($($x),*);
        std::process::exit(0);
    }};
}

fn main() {
    let args: Vec<String> = env::args().collect();

    enum InputType { Raw(String), File(String) }

    let mut input_type: Option<InputType> = Option::None;
    let mut hash_function: Option<&dyn Fn(Vec<u8>) -> String> = Option::None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-r" | "--raw" => {
                match input_type {
                    Some(_) => exit!("Can only specify -r/--raw or -f/--file once. Run hash --help for help"),
                    None => {
                        i += 1;
                        if i == args.len() { 
                            exit!("Must provide an argument after -r/--raw. Run hash --help for help") 
                        }
                        input_type = Some(InputType::Raw(args[i].to_string()));
                    }
                }
            },
            "-f" | "--file" => {
                match input_type {
                    Some(_) => exit!("Can only specify -r/--raw or -f/--file once. Run hash --help for help"),
                    None => {
                        i += 1;
                        if i == args.len() { 
                            exit!("Must provide an argument after -f/--file. Run hash --help for help") 
                        }
                        input_type = Some(InputType::File(args[i].to_string()));
                    }
                }
            },
            "-a" | "--algorithm" => match hash_function {
                Some(_) => exit!("Hash algorithm entered twice. Run hash --help for help"),
                None => { 
                    i += 1;
                    if i == args.len() { 
                        exit!("Must provide an argument after -a/--algorithm Run hash --help for help") 
                    }
                    hash_function = match args[i].as_str() {
                        "md5" => { Some(&md5) },
                        "sha1" => { Some(&sha1) },
                        "sha2-256" | "sha256" | "256" => { Some(&sha256) },
                        "sha2-512" | "sha512" | "512" => { Some(&sha512) },
                        "sha3-256" => { Some(&sha3_256) },
                        "sha3-512" => { Some(&sha3_512) },
                        _ => { exit!("Unrecognized algorithm {}. Run hash --help for help", args[i]); }
                    };
                },
            },
            "-h" | "--help" => 
                exit!("Usage: hash <options> \n\
                    \t-h | --help:\t\tdisplays this message.\n\n\
                    \t-r | --raw:\t\ttreat the next argument as raw text to be hashed. Cannot be used alongside -f/--file.\n\
                    \t-f | --file:\t\ttreat the next argument as a path to a file to be hashed. Cannot be used alongside -r/--raw.\n\
                        \t\tIf neither -r/--raw or -f/--file are specified, treats the first otherwise-unrecognized argument as raw text to be hashed.\n\n\
                    \t-a | --algorithm:\ttreat the next argument as the hashing algorithm to be used. Defaults to SHA2-256 if not specified. Supported hash algorithms:\n\
                            \t\tMD5 (md5)\n\
                            \t\tSHA-1 (sha1),\n\
                            \t\tSHA2-256 (sha2-256, sha256, 256) *DEFAULT*,\n\
                            \t\tSHA2-512 (sha2-512, sha512, 512),\n\
                            \t\tSHA3-256 (sha3-256),\n\
                            \t\tSHA3-512 (sha3-512)"),
            _ => {
                match input_type {
                    Some(_) => exit!("Unrecognized argument: {}. Run hash --help for help", args[i]),
                    None => { input_type = Some(InputType::Raw(args[i].to_string())); }
                }
            }  
        }

        i += 1;
    }

    let text: Vec<u8> = match input_type.unwrap_or_else(|| exit!("Didn't find anything to hash. Run hash --help for help")) {
        InputType::Raw(input) => input.as_bytes().to_vec(),
        InputType::File(input) => match read_file(&input) {
            Ok(text) => text,
            Err(e) => { exit!("Unable to read file: {}", e) }
        }
    };

    println!("{}", hash_function.unwrap_or(&sha256)(text));
}

fn read_file(filename: &str) -> io::Result<Vec<u8>> {
    let mut file = fs::File::open(filename)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}