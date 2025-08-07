use base64::prelude::*;
use crate::cli::Base64Format;
use std::io::{self,Read};
use std::fs::File;
pub fn process_encode(input: &str,format:Base64Format) -> anyhow::Result<()> {
    let engine = match format{
        Base64Format::Standard => BASE64_STANDARD,
        Base64Format::UrlSafe => BASE64_URL_SAFE_NO_PAD,
    };
    let mut reader:Box<dyn Read> = if input == "-" {
        Box::new(io::stdin())
    }else{
        Box::new(File::open(input)?)
    };
    let mut data = Vec::new();
    reader.read_to_end(&mut data)?;
    let data_str = String::from_utf8(data)?;
    let trimmed = data_str.trim();
    let encoded = engine.encode(trimmed);
    println!("Encoded: {}", encoded);
    Ok(())
}

pub fn process_decode(input: &str,format:Base64Format) -> anyhow::Result<()> {
    let engine = match format{
        Base64Format::Standard => BASE64_STANDARD,
        Base64Format::UrlSafe => BASE64_URL_SAFE_NO_PAD,
    };
    let mut reader:Box<dyn Read> = if input == "-" {
        Box::new(io::stdin())
    }else{
        Box::new(File::open(input)?)
    };
    let mut data = Vec::new();
    reader.read_to_end(&mut data)?;
    // 去除空白字符（包括换行符）
    let data_str = String::from_utf8(data)?;
    let trimmed = data_str.trim();
    let decoded = engine.decode(trimmed)?;
    let decoded_str = String::from_utf8(decoded)?;
    println!("Decoded: {}", decoded_str);
    Ok(())
}