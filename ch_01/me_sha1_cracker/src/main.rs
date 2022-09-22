use std::{env, error::Error};

use sha1::{Digest, Sha1};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

const SHA1_HEX_STRING_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() != 3 {
        println!("USAGE: shacracker: <wordlist.txt> <sha1_hash>");
        return Err("number of arguments".into());
    }

    let hash_to_crack = args[2].trim();
    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
        return Err("Sha1 hash of incorrect length".into());
    }

    let f = File::open(&args[1])?;
    let reader = BufReader::new(f);
    for line in reader.lines() {
        let common_pwd = line?;
        let mut hasher = Sha1::new();
        hasher.update(common_pwd.trim());
        let result = hasher.finalize();

        if hex::encode(result) == hash_to_crack {
            println!("sha1 of {:?} id {:?}", common_pwd, hash_to_crack);
            return Ok(());
        }
    }

    //let hex_string = hex::encode("Hello world!");

    //result[..]

    Ok(())
}
