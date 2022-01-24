use std::{env, fs, io::{self, Read}};
use hash::{Hasher, Sha256, Sha512};

fn main() {
    let args: Vec<String> = env::args().collect();

    enum InputType { Raw, File }

    let mut input_type: Option<InputType> = Option::None;
    let mut hasher: Option<Box<dyn Hasher>> = Option::None;
    let mut input: Option<String> = Option::None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-r" | "--raw" => {
                match input_type {
                    Some(_) => panic!("Must specify -r/--raw or -f/--file only once"),
                    None => {
                        input_type = Some(InputType::Raw);
                        input = Some((&args[i+1]).to_string());
                        i += 1;
                    }
                }
            },
            "-f" | "--file" => {
                match input_type {
                    Some(_) => panic!("Must specify -r/--raw or -f/--file only once"),
                    None => {
                        input_type = Some(InputType::File);
                        input = Some((&args[i+1]).to_string());
                        i += 1;
                    }
                }
            },
            "-t" | "--type" => match hasher {
                Some(_) => panic!("Hash type entered twice"),
                None => { 
                    hasher = match args[i+1].as_str() {
                        "256" | "sha256" => { Some(Box::new(Sha256)) },
                        "512" | "sha512" => { Some(Box::new(Sha512)) },
                        _ => { panic!("Acceptable hash types: sha256, sha512"); }
                    };
                    i += 1;
                },
            },
            _ => {
                match input_type {
                    Some(_) => panic!("Unrecognized argument: {}", args[i]),
                    None => {
                        input_type = Some(InputType::Raw);
                        input = Some((&args[i]).to_string());
                    }
                }
            }  
        }

        i += 1;
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