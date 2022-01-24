use std::{env, fs, io::{self, Read}};
use hash::{Hasher, Sha256, Sha512};

enum InputType { Raw, File }

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut input_type: Option<InputType> = Option::None;
    let mut hasher: Option<Box<dyn Hasher>> = Option::None;
    let mut input: Option<String> = Option::None;

    match args.len() {
        2 => { 
            input_type = Some(InputType::Raw);
            input = Some((&args[1]).to_string());
        },
        3 => match args[1].as_str() {
            "-r" | "--raw" => { 
                input_type = Some(InputType::Raw);
                input = Some((&args[2]).to_string());
            },
            "-f" | "--file" => { 
                input_type = Some(InputType::File);
                input = Some((&args[2]).to_string());
            },
            _ => { panic!("Acceptable input types:\n\tRaw contents of next arg (-r, --raw)\n\tFrom file (-f, --file)"); }
        },
        5 => {
            match args[1].as_str() {
                "-r" | "--raw" => { 
                    input_type = Some(InputType::Raw);
                    input = Some((&args[2]).to_string());
                },
                "-f" | "--file" => { 
                    input_type = Some(InputType::File);
                    input = Some((&args[2]).to_string());
                },
                "-t" | "--type" => match args[2].as_str() {
                    "256" | "sha256" => { hasher = Some(Box::new(Sha256)) },
                    "512" | "sha512" => { hasher = Some(Box::new(Sha512)) },
                    _ => { panic!("Acceptable hash types: sha256, sha512"); }
                },
                _ => { panic!("Acceptable input types:\n\tRaw contents of next arg (-r, --raw)\n\tFrom file (-f, --file)") }
            }

            match args[3].as_str() {
                "-r" | "--raw" => { 
                    match input_type {
                        Some(_) => panic!("Input type entered twice"),
                        None => { 
                            input_type = Some(InputType::Raw);
                            input = Some((&args[4]).to_string()); }
                    }
                },
                "-f" | "--file" => { 
                    match input_type {
                        Some(_) => panic!("Input type entered twice"),
                        None => { 
                            input_type = Some(InputType::File);
                            input = Some((&args[4]).to_string()); }
                    }
                },
                "-t" | "--type" => match hasher {
                    Some(_) => panic!("Hash type entered twice"),
                    None =>  match args[4].as_str() {
                        "256" | "sha256" => { hasher = Some(Box::new(Sha256)) },
                        "512" | "sha512" => { hasher = Some(Box::new(Sha512)) },
                        _ => { panic!("Acceptable hash types: sha256, sha512"); }
                    },
                }               
                _ => { panic!("Acceptable input types:\n\tRaw contents of next arg (-r, --raw)\n\tFrom file (-f, --file)") }
            }
        },
        _ => { panic!("Usage: hash <INPUT TYPE> [INPUT]"); }
    }

    let input_type: InputType = input_type.unwrap_or_else(|| panic!("No input type specified"));
    let input: String = input.unwrap_or_else(|| panic!("No input specified"));
    let hasher: Box<dyn Hasher> = hasher.unwrap_or(Box::new(Sha256));

    let text: Vec<u8> = match input_type {
        InputType::Raw => input.as_bytes().to_vec(),
        InputType::File => match read_file(&input) {
            Ok(contents) => contents,
            Err(e) => panic!("Error reading file: {}", e)
        }
    };

    println!("{}", hasher.hash(text));
}

fn read_file(filename: &str) -> io::Result<Vec<u8>> {
    let mut file = fs::File::open(filename)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}