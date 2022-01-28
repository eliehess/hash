use std::{env, fs, io::{self, Read}};
use hash::{Hasher, 
    sha2::{Sha256, Sha512},
    sha3::{Sha3_256, Sha3_512},
    md5::MD5
};

fn main() {
    let args: Vec<String> = env::args().collect();

    enum InputType { Raw(String), File(String) }

    let mut input_type: Option<InputType> = Option::None;
    let mut hasher: Option<Box<dyn Hasher>> = Option::None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-r" | "--raw" => {
                match input_type {
                    Some(_) => panic!("Can only specify -r/--raw or -f/--file once"),
                    None => {
                        i += 1;
                        if i == args.len() { 
                            panic!("Must provide an argument after -r/--raw") 
                        }
                        input_type = Some(InputType::Raw(args[i].to_string()));
                    }
                }
            },
            "-f" | "--file" => {
                match input_type {
                    Some(_) => panic!("Can only specify -r/--raw or -f/--file once"),
                    None => {
                        i += 1;
                        if i == args.len() { 
                            panic!("Must provide an argument after -f/--file") 
                        }
                        input_type = Some(InputType::File(args[i].to_string()));
                    }
                }
            },
            "-a" | "--algorithm" => match hasher {
                Some(_) => panic!("Hash algorithm entered twice"),
                None => { 
                    i += 1;
                    if i == args.len() { 
                        panic!("Must provide an argument after -a/--algorithm") 
                    }
                    hasher = match args[i].as_str() {
                        "256" | "sha256" | "sha2-256" => { Some(Box::new(Sha256)) },
                        "512" | "sha512" | "sha2-512" => { Some(Box::new(Sha512)) },
                        "sha3-256" => { Some(Box::new(Sha3_256)) },
                        "sha3-512" => { Some(Box::new(Sha3_512)) },
                        "md5" => { Some(Box::new(MD5)) },
                        _ => { panic!("Supported hash algorithms:\n\
                            \tSHA-256 (sha2-256, sha256, 256) *DEFAULT*,\n\
                            \tSHA-512 (sha2-512, sha512, 512),\n\
                            \tSHA3-256 (sha3-256),\n\
                            \tSHA3-512 (sha3-512)\n\
                            \tMD5 (md5)"); }
                    };
                },
            },
            _ => {
                match input_type {
                    Some(_) => panic!("Unrecognized argument: {}", args[i]),
                    None => { input_type = Some(InputType::Raw(args[i].to_string())); }
                }
            }  
        }

        i += 1;
    }

    let text: Vec<u8> = match input_type.expect("No input type specified") {
        InputType::Raw(input) => input.as_bytes().to_vec(),
        InputType::File(input) => read_file(&input).expect("Unable to read file")
    };

    let result: String = hasher.unwrap_or_else(|| Box::new(Sha256)).hash(text);

    println!("{}", result);
}

fn read_file(filename: &str) -> io::Result<Vec<u8>> {
    let mut file = fs::File::open(filename)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}