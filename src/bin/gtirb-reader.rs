use anyhow::Result;

pub fn main() -> Result<()> {
    println!("gtirb");
    let path = "/home/kwarrick/ex.gtirb";
    let ir = gtirb::read(path)?;
    for module in ir.modules() {
        eprintln!("{:?}", module.uuid());
    }
    eprintln!("{:#?}", ir);
    Ok(())
}
