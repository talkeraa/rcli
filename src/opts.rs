use clap::{Parser};
use std::path::Path;
use csv::Reader;
use serde::{Deserialize,Serialize};
use anyhow;
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
}
#[derive(Debug,Parser)] 
pub struct CsvOpts{
    #[arg(short,long,default_value="input.csv",value_parser=verify_input_file)]
    pub input:String,
    #[arg(short,long,default_value="output.csv")]
    pub output:String,
    #[arg(short,long,default_value_t=',')]
    pub delimiter:char,
    #[arg(short='g',long,default_value_t=true)]
    pub has_header:bool,
}
pub fn verify_input_file(input:&str )->Result<String,String>{
  if std::path::Path::new(input).exists(){
    Ok(input.to_string())
  }else{
    Err(format!("文件不存在:{}",input))
  }
}