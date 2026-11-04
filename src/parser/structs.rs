use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Clone)]
pub struct Reloc {
    pub offset: u64,
    pub kind: u32,
    pub sym: u32,
    pub addend: i64,
}

#[derive(Debug, Clone)]
pub struct Section {
    pub name: String,
    pub addr: u64,
    pub size: u64,
    pub offset: u64,
    pub in_file: bool,
    pub in_mem: bool,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub start: u64,
    pub end: u64,
}

#[derive(Debug, Clone, Default)]
pub struct Binary {
    pub data: Vec<u8>,
    pub sections: Vec<Section>,
    pub functions: Vec<Function>,
    pub imports: HashMap<u64, String>,
    pub entry: Option<u64>,
    pub stripped: bool,
}

impl Binary {
    pub fn section_at(&self, addr: u64) -> Option<&Section> {
        return self
            .sections
            .iter()
            .find(|s| addr >= s.addr && addr - s.addr < s.size);
    }

    pub fn function_at(&self, addr: u64) -> Option<&Function> {
        return self
            .functions
            .iter()
            .find(|f| addr >= f.start && addr <= f.end);
    }

    pub fn read(&self, addr: u64, len: usize) -> Option<&[u8]> {
        let s = self.section_at(addr)?;
        if !s.in_file {
            return None;
        }

        let rel = addr - s.addr;
        if rel.checked_add(len as u64)? > s.size {
            return None;
        }
        
        let start = s.offset.checked_add(rel)? as usize;
        let end = start.checked_add(len)?;
        return self.data.get(start..end);
    }
}
