use clap::Parser;
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