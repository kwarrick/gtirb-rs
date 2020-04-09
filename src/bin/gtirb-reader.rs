use anyhow::Result;

pub fn main() -> Result<()> {
    println!("gtirb");
    let path = "/home/kwarrick/Linux/out.ir";
    let ir = gtirb::read(path)?;
    eprintln!("{:#?}", ir);
    Ok(())
}
