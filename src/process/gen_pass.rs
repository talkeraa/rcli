use std::str::from_utf8;

use crate::cli::genpass::GenPassOpts;
use anyhow;
use rand::Rng;
use rand::seq::{IndexedRandom, SliceRandom};
const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const NUMBERS: &[u8] = b"0123456789";
const SYMBOLS: &[u8] = b"!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

pub fn process_genpass(opts:GenPassOpts)->Result<(),anyhow::Error>{
    let mut rng=rand::rng();
    let mut chars = Vec::new();
    let mut password=Vec::new();
    if opts.uppercase{
        chars.extend_from_slice(UPPERCASE);
        password.push(*UPPERCASE.choose(&mut rng).unwrap());
    }
    if opts.lowercase{
        chars.extend_from_slice(LOWERCASE);
        password.push(*LOWERCASE.choose(&mut rng).unwrap());
    }
    if opts.number{
        chars.extend_from_slice(NUMBERS);
        password.push(*NUMBERS.choose(&mut rng).unwrap() );
    }
    if opts.symbols{
        chars.extend_from_slice(SYMBOLS);
        password.push(*SYMBOLS.choose(&mut rng).unwrap());
    }
    let len=opts.length-password.len() as u8;
    for _ in 0..len{
        let index=rng.random_range(0..chars.len());
        password.push(chars[index] );
    }
    password.shuffle(&mut rng);
   // println!("{}",password.into_iter().collect::<String>());
    println!("{}",String::from_utf8(password)?);
    Ok(())
}