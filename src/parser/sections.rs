use anyhow::Error;
use object::read::elf::SectionHeader;
use object::read::elf::SectionTable;
use object::{Endianness, elf};

use super::structs::Section;

pub fn parse_sect(
    table: &SectionTable<elf::FileHeader64<Endianness>>,
    endian: Endianness,
) -> Result<Vec<Section>, Error> {
    let mut out: Vec<Section> = Vec::new();
    for section in table.iter() {
        let name = String::from_utf8_lossy(table.section_name(endian, section)?).into_owned();
        out.push(Section {
            name,
            addr: section.sh_addr(endian),
            size: section.sh_size(endian),
            offset: section.sh_offset(endian),
            in_file: section.sh_type(endian) != elf::SHT_NOBITS,
            in_mem: section.sh_flags(endian).contains(elf::SHF_ALLOC),
        });
    }

    return Ok(out);
}
