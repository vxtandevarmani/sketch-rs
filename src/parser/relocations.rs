use std::collections::HashMap;

use anyhow::{Context, Error};
use object::{
    Endianness, read::elf::{Rela, SectionHeader},
};

use object::elf::{self};
use object::read::elf::SectionTable;
use object::read::elf::SymbolTable;
use super::structs::*;

pub fn read_locs(
    sections: &SectionTable<elf::FileHeader64<Endianness>>,
    name: &str,
    data: &[u8],
    endian: Endianness,
) -> Result<Vec<Reloc>, Error> {

    let mut out: Vec<Reloc> = Vec::new();

    let Some((_, sect)) = sections.section_by_name(endian, name.as_bytes()) else {
        return Ok(out);
    };

    let (relas, _) = sect
        .rela(endian, data)
        .context("[-] invalid relocations")?
        .context("[-] relocations not found")?;

    for entry in relas {
        out.push(Reloc {
            offset: entry.r_offset(endian),
            kind: entry.r_type(endian, false).0,
            sym: entry.r_sym(endian, false),
            addend: entry.r_addend(endian),
        });
    }


    return Ok(out);
}


fn dynsym_name(
    dynsyms: &SymbolTable<elf::FileHeader64<Endianness>>,
    idx: u32,
    endian: Endianness
) -> Result<String, Error> {

    let sym = dynsyms.symbol(object::SymbolIndex(idx as usize))?;
    return Ok(String::from_utf8_lossy(dynsyms.symbol_name(endian, sym)?).into_owned());
}


pub fn build_imports(
    dyn_relocs: &[Reloc],
    plt_reloc: &[Reloc],
    dynsyms: &SymbolTable<elf::FileHeader64<Endianness>>,
    endian: Endianness
) -> Result<HashMap<u64, String>, Error> {
    let mut imports = HashMap::new();
    for r in plt_reloc.iter().chain(dyn_relocs) {
        if r.sym != 0 {
            imports.insert(r.offset, dynsym_name(dynsyms, r.sym, endian)?);
        }
    }

    Ok(imports)
}