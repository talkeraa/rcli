use std::fmt;
use std::str::FromStr;
use anyhow;
use clap::Parser;
use super::verify_input_file;
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