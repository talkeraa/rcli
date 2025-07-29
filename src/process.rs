use csv::Reader;
use serde::{Deserialize,Serialize};
use anyhow;
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

pub fn process_csv(input:&str,output:&str)->Result<(),anyhow::Error>{
    let mut reader=Reader::from_path(input)?;
    let mut ret=Vec::with_capacity(128);
    for result in reader.deserialize(){
        let record:CsvData=result?;
        ret.push(record);
    }
    let json=serde_json::to_string_pretty(&ret)?;
    std::fs::write(output,&json)?;
    Ok(())
}