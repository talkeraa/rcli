use rcli::{Opst,Subcommand,process_csv,process_genpass,Base64Subcommand};
use clap::Parser;
use rcli::{process_encode,process_decode};
fn main() -> anyhow::Result<()>{
    let opts=Opst::parse();
    match opts.cmd{
        Subcommand::Csv(csv_opts)=>{
            let output = if let Some(output)=csv_opts.output{
                output
            }else{
                format!("output.{}",csv_opts.fotmat)
            };
            process_csv(&csv_opts.input,&output,csv_opts.fotmat)?;
        }
        Subcommand::GenPass(gen_pass_opts)=>{
            process_genpass(gen_pass_opts)?;
        }
        Subcommand::Base64(base64_opts)=>{
            match base64_opts.cmd{
                Base64Subcommand::Encode(encode_opts)=>{
                    process_encode(&encode_opts.input,encode_opts.format)?;
                }
                Base64Subcommand::Decode(decode_opts)=>{
                    process_decode(&decode_opts.input,decode_opts.format)?;
                }
            }
        }
    }
    Ok(())
}
