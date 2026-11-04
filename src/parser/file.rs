use anyhow::{Error, ensure, Result};
use object::{Endianness, elf::{self, ELFCLASS64, ELFMAG, EV_CURRENT}, read::elf::FileHeader};
use super::{
    structs::*,
    sections::parse_sect,
    functions::parse_fns,
};

fn validate_elf(header: &elf::FileHeader64<Endianness>, endian: Endianness) -> Result<(), Error> {
    let ident: &elf::Ident = header.e_ident();
    let ty: elf::FileType = header.e_type(endian);

    ensure!(ident.magic == ELFMAG, "[-] No Magic bytes");
    ensure!(ident.version == EV_CURRENT, "[-] Invalid ELF version");
    ensure!(ty == elf::ET_EXEC || ty == elf::ET_DYN, "[-] Not an executable!");
    ensure!(ident.class == ELFCLASS64       &&
            ident.data == elf::ELFDATA2LSB  &&
            header.e_machine(endian) == elf::EM_X86_64,
            "[-] Not X86_64 Little Endian"
    );



    Ok(())
}



pub fn load(path: &str) -> Result<Binary, Error> {

    let data_out = std::fs::read(path)?;
    let elf = elf::FileHeader64::<Endianness>::parse(&*data_out)?;
    let endian = elf.endian()?;
    let _ = validate_elf(elf, endian)?;
    let entry = elf.e_entry(endian);

    let sections = elf.sections(endian, &*data_out)?;
    let symbols = sections.symbols(endian, &*&data_out, elf::SHT_SYMTAB)?;
    let dynsyms = sections.symbols(endian, &*&data_out, elf::SHT_DYNSYM)?;
    let stripped = symbols.is_empty();

    let section_out = parse_sect(&sections, endian)?;
    let eh = section_out.iter().find(|s| s.name == "eh.frame");
    let function_out = parse_fns(&symbols, eh, &*&data_out, endian, stripped)?;

    Ok(Binary::default())
}