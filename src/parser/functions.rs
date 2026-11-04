use super::structs::{Function, Section};
use anyhow::{Context, Error, ensure};
use object::elf::{self};
use object::{
    Endianness,
    read::elf::{Sym, SymbolTable},
};

use gimli::{BaseAddresses, CieOrFde, EhFrame, LittleEndian, UnwindSection};

fn parse_fn_eh_frame(eh: &Section, data: &[u8]) -> Result<Vec<Function>, Error> {
    let mut out: Vec<Function> = Vec::new();
    let start = eh.offset as usize;
    let end = start
        .checked_add(eh.size as usize)
        .context("[-] .eh_frame size overflow")?;
    let bytes = data
        .get(start..end)
        .context("[-] .eh_frame out of bounds")?;

    let eh_frame = EhFrame::new(bytes, LittleEndian);
    let bases = &BaseAddresses::default().set_eh_frame(eh.addr);
    let mut entries = eh_frame.entries(&bases);

    while let Some(entry) = entries.next()? {
        if let CieOrFde::Fde(partial) = entry {
            let fde = partial.parse(EhFrame::cie_from_offset)?;
            let start = fde.initial_address();
            let size = fde.len();
            let end = start.checked_add(size).context("[-] fde entry overflow")?;
            out.push(Function {
                name: format!("sub_{start:x}"),
                start: start,
                end: end,
            });
        }
    }

    out.sort_by_key(|f| f.start);
    out.dedup_by_key(|f| f.start);

    return Ok(out);
}

fn parse_fn_symtab(
    symtable: &SymbolTable<elf::FileHeader64<Endianness>>,
    fntable: &mut [Function],
    endian: Endianness,
) -> Result<(), Error> {
    for sym in symtable.iter() {
        if sym.st_type() != elf::STT_FUNC || sym.is_undefined(endian) {
            continue;
        }
        let name = String::from_utf8_lossy(symtable.symbol_name(endian, sym)?).into_owned();
        let start = sym.st_value(endian);
        match fntable.binary_search_by_key(&start, |f| f.start) {
            Ok(i) => fntable[i].name = name,
            Err(_) => continue,
        }
    }

    return Ok(());
}

pub fn parse_fns(
    table: &SymbolTable<elf::FileHeader64<Endianness>>,
    sections: &[Section],
    data: &[u8],
    endian: Endianness,
    is_stripped: bool,
) -> Result<Vec<Function>, Error> {
    let eh = sections.iter().find(|s| s.name == ".eh_frame");
    ensure!(eh.is_some(), "[-] .eh_frame is not present within binary");
    let mut out: Vec<Function> = parse_fn_eh_frame(eh.unwrap(), data)?;

    if !is_stripped {
        let _ = parse_fn_symtab(table, &mut out, endian)?;
    }

    return Ok(out);
}
