use anyhow::Result;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "gtirb-reader")]
struct Opt {
    #[structopt(name = "BIN", parse(from_os_str))]
    files: Vec<PathBuf>,
}

pub fn main() -> Result<()> {
    let opt = Opt::from_args();
    for file in opt.files {
        let (_, ir) = gtirb::read(file)?;
        for module in ir.modules() {
            eprintln!("{:#?}", module);
        }
    }
    Ok(())
}
