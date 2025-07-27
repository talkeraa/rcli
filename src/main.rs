use clap::{Parser};
use std::path::Path;
use csv::Reader;
use serde::{Deserialize,Serialize};
#[derive(Debug,Parser)]
#[command(name="rcli",author, version, about, long_about = None)]
struct Opst{
    #[command(subcommand)]
    cmd:Subcommand,
}
#[derive(Debug,Deserialize,Serialize)]
struct CsvData{
    name:String,
    position:String,
    dob:String,
    nationnality:String,
    kit: u8,
}
#[derive(Debug,Parser)]
enum Subcommand{
    #[command(name="csv",about="csv操作")]
    Csv(CsvOpts),
}
#[derive(Debug,Parser)] 
struct CsvOpts{
    #[arg(short,long,default_value="input.csv",value_parser=verify_input_file)]
    input:String,
    #[arg(short,long,default_value="output.csv")]
    output:String,
    #[arg(short,long,default_value_t=',')]
    delimiter:char,
    #[arg(short='g',long,default_value_t=true)]
    has_header:bool,
}
fn verify_input_file(input:&str )->Result<String,String>{
  if std::path::Path::new(input).exists(){
    Ok(input.to_string())
  }else{
    Err(format!("文件不存在:{}",input))
  }
}
fn main() {
    let opts=Opst::parse();
    println!("{:?}",opts);
    match opts.cmd{
        Subcommand::Csv(csv_opts)=>{
            let mut reader=Reader::from_path(csv_opts.input).unwrap();
            for result in reader.records(){
                let record=result.unwrap();
                println!("{:?}",record);
            }
        }
    }
}
