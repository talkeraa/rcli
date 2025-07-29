use rcli::{Opst,Subcommand,process_csv};
use clap::Parser;
fn main() -> anyhow::Result<()>{
    let opts=Opst::parse();
    match opts.cmd{
        Subcommand::Csv(csv_opts)=>{
            process_csv(&csv_opts.input,&csv_opts.output)?;
        }
    }
    Ok(())
}
