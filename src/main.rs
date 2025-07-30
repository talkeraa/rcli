use rcli::{Opst,Subcommand,process_csv};
use clap::Parser;
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
    }
    Ok(())
}
