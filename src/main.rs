use anyhow::Error;
use sketch_rs::parser;

fn main() -> Result<(), Error>{
    let argv: Vec<String> = std::env::args().collect();
    if argv.len() != 2 {
        eprintln!("Usage {} <BINARY>", argv[0]);
    }

    let path = &argv[1];
    let bin = parser::file::load(path)?;


    Ok(())
}
