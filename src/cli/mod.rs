mod csv;
pub mod genpass;
mod base64;
pub use self::csv::CsvOpts;
pub use self::genpass::GenPassOpts;
pub use self::base64::Base64Opts;
use clap::{Parser};
pub use self::csv::OutPutFormat;
pub use self::base64::*;
use std::path::Path;
#[derive(Debug,Parser)]
#[command(name="rcli",author, version, about, long_about = None)]
pub struct Opst{
    #[command(subcommand)]
    pub cmd:Subcommand,
}

#[derive(Debug,Parser)]
pub enum Subcommand{
    #[command(name="csv",about="csv操作")]
    Csv(CsvOpts),
    #[command(name="genpass",about="生成密码")]
    GenPass(GenPassOpts),
    #[command(name="base64",about="base64操作")]
    Base64(Base64Opts),
}


pub fn verify_input_file(input:&str )->Result<String,String>{
   if input=="-"|| Path::new(input).exists(){
    Ok(input.into())
   }else{
    Err(format!("文件不存在:{}",input))
   }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_verify_input_file(){
        assert_eq!(verify_input_file("-"),Ok("-".into()));
        assert_eq!(verify_input_file("Cargo.toml"),Ok("Cargo.toml".into()));
        assert_eq!(verify_input_file("input.txt"),Err("文件不存在:input.txt".to_string()));
    }
}