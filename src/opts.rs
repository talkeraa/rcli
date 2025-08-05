use clap::{Parser};
use std::path::Path;
use csv::Reader;
use serde::{Deserialize,Serialize};
use anyhow;
use std::str::FromStr;
use std::fmt;
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
}
#[derive(Debug,Parser)]
pub struct GenPassOpts{
    #[arg(short,long,default_value="10")]
    pub length:u8,
    #[arg(short,long,default_value_t=true)]
    pub number:bool,
    #[arg(long,default_value_t=true)]
    pub symbols:bool,
    #[arg(long,default_value_t=true)]
    pub uppercase:bool,
    #[arg(long,default_value_t=true)]
    pub lowercase:bool,
}

#[derive(Debug,Clone,Copy)]
pub enum OutPutFormat{
    Json,
    Toml,
    Yaml,
}
#[derive(Debug,Parser)] 
pub struct CsvOpts{
    #[arg(short,long,default_value="input.csv",value_parser=verify_input_file)]
    pub input:String,
    #[arg(short,long)]
    pub output:Option<String>,
    #[arg(short,long,value_parser=parse_output_format,default_value="json")]
    pub fotmat:OutPutFormat,
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

pub fn parse_output_format(input:&str)->Result<OutPutFormat,anyhow::Error>{
   input.parse::<OutPutFormat>()
}

impl From<OutPutFormat> for &'static str{
    fn from(format:OutPutFormat)->Self{
        match format{
            OutPutFormat::Json=>"json",
            OutPutFormat::Toml=>"toml",
            OutPutFormat::Yaml=>"yaml",
        }
    }
}

impl FromStr for OutPutFormat{
    type Err=anyhow::Error;
    fn from_str(value: &str)->Result<Self,Self::Err>{
        match value.to_lowercase().as_str(){
            "json"=>Ok(OutPutFormat::Json),
            "toml"=>Ok(OutPutFormat::Toml),
            "yaml"=>Ok(OutPutFormat::Yaml),
            _=>Err(anyhow::anyhow!("无效的输出格式:{}",value))
        }
    }
}

impl fmt::Display for OutPutFormat{
    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{
        write!(f,"{}",Into::<&'static str>::into(*self))
    }
}