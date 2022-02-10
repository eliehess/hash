use std::{env, fs, collections::VecDeque, io::{self, Read}};
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

fn process_args(mut args: VecDeque<String>) -> Result<(InputType, impl Fn(Vec<u8>) -> String), String> {
    let mut input_type: Option<InputType> = Option::None;
    let mut backup_input: Option<String> = Option::None;
    let mut hash_function: Option<&dyn Fn(Vec<u8>) -> String> = Option::None;

    // The first argument is the executable name, which can be discarded
    args.pop_front();
    while let Some(mut arg) = args.pop_front() {
        match arg.as_str() {
            "-r" | "--raw" => 
                match input_type {
                    Some(_) => return Err("Can only specify -r/--raw or -f/--file once. Run hash --help for help".to_string()),
                    None => {
                        // Consume next argument (raw input to hash)
                        arg = match args.pop_front() {
                            None => return Err("Must provide an argument after -r/--raw. Run hash --help for help".to_string()),
                            Some(x) => x
                        };
                        input_type = Some(InputType::Raw(arg));
                    }
                }
            "-f" | "--file" => 
                match input_type {
                    Some(_) => return Err("Can only specify -r/--raw or -f/--file once. Run hash --help for help".to_string()),
                    None => {
                        // Consume next argument (path to file to hash)
                        arg = match args.pop_front() {
                            None => return Err("Must provide an argument after -f/--file. Run hash --help for help".to_string()),
                            Some(x) => x
                        };
                        input_type = Some(InputType::File(arg));
                    }
                }
            "-a" | "--algorithm" => 
                match hash_function {
                    Some(_) => return Err("Hash algorithm entered twice. Run hash --help for help".to_string()),
                    None => { 
                        // Consume next argument (name of algorithm)
                        arg = match args.pop_front() {
                            None => return Err("Must provide an argument after -a/--algorithm. Run hash --help for help".to_string()),
                            Some(x) => x
                        };
                        hash_function = Some(match arg.as_str() {
                            "md5" => &md5,
                            "sha1" => &sha1,
                            "sha2-256" | "sha256" | "256" => &sha256,
                            "sha2-512" | "sha512" | "512" => &sha512,
                            "sha3-256" => &sha3_256,
                            "sha3-512" => &sha3_512,
                            "tiger" => &tiger,
                            "whirlpool" => &whirlpool,
                            _ => { return Err(format!("Unrecognized algorithm \"{}\". Run hash --help for help", arg)); }
                        });
                    }
                }
            "-h" | "--help" => 
                return Err("Usage: hash <options> \n\
                    \t-h | --help:\t\tdisplays this message.\n\n\
                    \t-r <input> | --raw <input>:\t\thash the specified raw input. Cannot be used alongside -f/--file.\n\
                    \t-f <filename> | --file <filename>:\t\thash the specified file. Cannot be used alongside -r/--raw.\n\
                        \t\tIf neither -r/--raw or -f/--file are specified, treats the first otherwise-unrecognized argument as raw text to be hashed.\n\n\
                    \t-a <alg> | --algorithm <alg>:\tuse the specified hash algorithm. Defaults to SHA2-256 if not specified. Supported hash algorithms:\n\
                            \t\tMD5 (md5)\n\
                            \t\tSHA-1 (sha1),\n\
                            \t\tSHA2-256 (sha2-256, sha256, 256) *DEFAULT*,\n\
                            \t\tSHA2-512 (sha2-512, sha512, 512),\n\
                            \t\tSHA3-256 (sha3-256),\n\
                            \t\tSHA3-512 (sha3-512),\n\
                            \t\tTiger (tiger),\n\
                            \t\tWhirlpool (whirlpool)".to_string()),
            _ =>
                match backup_input {
                    // If there's only one otherwise-unrecognized argument, treat it as raw input to hash. More than one is an error
                    Some(_) => return Err(format!("Unrecognized argument \"{}\". Run hash --help for help", arg)),
                    None => { backup_input = Some(arg); }
                }
        }
    }

    // Exactly one of regular input or backup input can be specified
    match input_type {
        None => match backup_input {
            None => Err("Didn't find anything to hash. Run hash --help for help".to_string()),
            Some(backup) => Ok((InputType::Raw(backup), hash_function.unwrap_or(&sha256)))
        },
        Some(input_type) => match backup_input {
            None => Ok((input_type, hash_function.unwrap_or(&sha256))),
            Some(backup) => Err(format!("Unrecognized argument \"{}\". Run hash --help for help", backup))
        }
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