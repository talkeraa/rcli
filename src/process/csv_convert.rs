use csv::Reader;
use serde::{Deserialize,Serialize};
use serde_json::Value;
use anyhow;

use crate::OutPutFormat;
#[derive(Debug,Deserialize,Serialize)]
pub struct CsvData{
    #[serde(rename="Name")]
    name:String,
    #[serde(rename="Position")]
    position:String,
    #[serde(rename="DOB")]
    dob:String,
    #[serde(rename="Nationality")]
    nationality:String,
    #[serde(rename="Kit Number")]
    kit: u8,
}

pub fn process_csv(input:&str,output:&str,format:OutPutFormat)->Result<(),anyhow::Error>{
    let mut reader=Reader::from_path(input)?;
    let mut ret=Vec::with_capacity(128);
    let headers=reader.headers()?.clone();
    for result in reader.records(){
        let record=result?;
        let json_value=headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
    }
    let content = match format{
        OutPutFormat::Json=>serde_json::to_string_pretty(&ret)?,
        OutPutFormat::Toml=>toml::to_string(&ret)?,
        OutPutFormat::Yaml=>serde_yaml::to_string(&ret)?,
    };
    std::fs::write(output,&content)?;
    println!("转换完成，已保存到{}",output);
    Ok(())
}