use std::{env, fs, io::{self, Read}};
use hash::{
    md5::md5,
    sha1::sha1,
    sha2::{sha256, sha512},
    sha3::{sha3_256, sha3_512},
    tiger::tiger,
    whirlpool::whirlpool
};

enum InputType { Raw(String), File(String) }

fn main() {
    match process_args(env::args().collect()) {
        Err(e) => println!("{}", e),
        Ok(tuple) => {
            let (input_type, hash_function) = tuple;
            match get_text(input_type) {
                Err(e) => println!("{}", e),
                Ok(text) => println!("{}", hash_function(text))
            }
        }
    }
}

fn process_args(args: Vec<String>) -> Result<(InputType, impl Fn(Vec<u8>) -> String), String> {
    let mut input_type: Option<InputType> = Option::None;
    let mut hash_function: Option<&dyn Fn(Vec<u8>) -> String> = Option::None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-r" | "--raw" => {
                match input_type {
                    Some(_) => return Err("Can only specify -r/--raw or -f/--file once. Run hash --help for help".to_string()),
                    None => {
                        i += 1;
                        if i == args.len() { 
                            return Err("Must provide an argument after -r/--raw. Run hash --help for help".to_string()) 
                        }
                        input_type = Some(InputType::Raw(args[i].to_string()));
                    }
                }
            },
            "-f" | "--file" => {
                match input_type {
                    Some(_) => return Err("Can only specify -r/--raw or -f/--file once. Run hash --help for help".to_string()),
                    None => {
                        i += 1;
                        if i == args.len() { 
                            return Err("Must provide an argument after -f/--file. Run hash --help for help".to_string()) 
                        }
                        input_type = Some(InputType::File(args[i].to_string()));
                    }
                }
            },
            "-a" | "--algorithm" => match hash_function {
                Some(_) => return Err("Hash algorithm entered twice. Run hash --help for help".to_string()),
                None => { 
                    i += 1;
                    if i == args.len() { 
                        return Err("Must provide an argument after -a/--algorithm Run hash --help for help".to_string()) 
                    }
                    hash_function = Some(match args[i].as_str() {
                        "md5" => &md5,
                        "sha1" => &sha1,
                        "sha2-256" | "sha256" | "256" => &sha256,
                        "sha2-512" | "sha512" | "512" => &sha512,
                        "sha3-256" => &sha3_256,
                        "sha3-512" => &sha3_512,
                        "tiger" => &tiger,
                        "whirlpool" => &whirlpool,
                        _ => { return Err(format!("Unrecognized algorithm {}. Run hash --help for help", args[i])); }
                    });
                },
            },
            "-h" | "--help" => 
            return Err("Usage: hash <options> \n\
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
                            \t\tSHA3-512 (sha3-512),\n\
                            \t\tTiger (tiger),\n\
                            \t\tWhirlpool (whirlpool)".to_string()),
            _ => {
                match input_type {
                    Some(_) => return Err(format!("Unrecognized argument: {}. Run hash --help for help", args[i])),
                    None => { input_type = Some(InputType::Raw(args[i].to_string())); }
                }
            }  
        }

        i += 1;
    }

    match input_type {
        None => Err("Didn't find anything to hash. Run hash --help for help".to_string()),
        Some(input_type) => Ok((input_type, hash_function.unwrap_or(&sha256)))
    }
}

fn get_text(input_type: InputType) -> Result<Vec<u8>, String> {
    match input_type {
        InputType::Raw(input) => Ok(input.as_bytes().to_vec()),
        InputType::File(input) => match read_file(&input) {
            Ok(text) => Ok(text),
            Err(e) => { Err(format!("Unable to read file: {}", e)) }
        }
    }
}

fn read_file(filename: &str) -> io::Result<Vec<u8>> {
    let mut file = fs::File::open(filename)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}