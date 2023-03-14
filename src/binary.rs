use super::{format_err, reader_bail};
use crate::util::error::{WasmError, WasmResult};
use byteorder::{LittleEndian, ReadBytesExt};
use std::marker;
use std::{collections::btree_map::VacantEntry, io::Read};
use varintrs::{Binary, ReadBytesVarExt};

const SECTION_CUSTOM: u8 = 0;
const SECTION_TYPE: u8 = 1;
const SECTION_IMPORT: u8 = 2;
const SECTION_FUNCTION: u8 = 3;
const SECTION_TABLE: u8 = 4;
const SECTION_MEMORY: u8 = 5;
const SECTION_GLOBAL: u8 = 6;
const SECTION_EXPORT: u8 = 7;
const SECTION_START: u8 = 8;
const SECTION_ELEMENT: u8 = 9;
const SECTION_CODE: u8 = 10;
const SECTION_DATA: u8 = 11;
const SECTION_DATA_COUNT: u8 = 12;
const SECTION_TAG: u8 = 13;

pub enum Components {
    TypeSection(),
    FunctionSection(FunctionType),
}

struct Module {
    Magic: u32,
    Version: u32,
    //type_section: Box<[FunctionType]>,
}

impl<'a> Module {
    fn read_type_sec(&mut self, reader: &mut BinaryReader<'a>) -> WasmResult<()> {
        let func_type_count = reader.read_var_u32()? as usize;
        let func_type: Vec<FunctionType> = Vec::with_capacity(func_type_count);
        Ok(())
    }
}

pub enum ValType {
    I32,
    I64,
    F32,
    F64,
    V128,
    ExternRef,
    FuncRef,
}

impl ValType {
    pub(crate) fn from_byte(byte: u8) -> Option<ValType> {
        match byte {
            0x7F => Some(ValType::I32),
            0x7E => Some(ValType::I64),
            0x7D => Some(ValType::F32),
            0x7C => Some(ValType::F64),
            0x7B => Some(ValType::V128),
            0x70 => Some(ValType::FuncRef),
            0x6F => Some(ValType::ExternRef),
            _ => None,
        }
    }
}

pub trait ComponentsReader<'a>: Sized {
    fn from_reader(reader: &mut BinaryReader<'a>) -> WasmResult<Self>;
}

impl<'a> ComponentsReader<'a> for ValType {
    fn from_reader(reader: &mut BinaryReader<'a>) -> WasmResult<Self> {
        match ValType::from_byte(reader.peek()?) {
            Some(vt) => Ok(vt),
            None => {
                reader_bail!(1, "invalid value type")
            }
        }
    }
}

pub enum Type {
    Func(FunctionType),
}

pub struct FunctionType {
    params: Box<[ValType]>,
    returns: Box<[ValType]>,
}

impl<'a> FunctionType {
    fn from_reader(reader: &mut BinaryReader<'a>) {
        // let byte_count = reader.read_var_u32();
        // 读取 type 大小
        //     let type_count = reader.rusize();
        //  //
        // let vec: Vec<FunctionType> = Vec::with_capacity(type_count);
    }
}

struct Import {}

impl<'a> Import {
    fn from_reader(reader: &mut BinaryReader<'a>) {
        reader.read_var_u32();
    }
}

impl Module {
    fn from_binary(data: &[u8]) -> Module {
        if data.len() < 4 {
            panic!("unexpected end of magic header");
        }
        Self {
            Magic: 0,
            Version: 0,
            // type_section: 0,
        }
    }

    fn from_file() {}

    fn new() {}
}

// section 读取
struct SectionReader<'a, T> {
    reader: BinaryReader<'a>, //二进制文件读取
    count: usize,             // 有多少个 section
    _marker: marker::PhantomData<T>,
}

impl<'a, T> SectionReader<'a, T> {
    fn new(data: &'a [u8]) -> WasmResult<Self> {
        let mut reader = BinaryReader::new(data);
        let count = reader.rusize()?;
        Ok(SectionReader {
            reader,
            count,
            _marker: marker::PhantomData,
        })
    }
}

impl<'a, T> IntoIterator for SectionReader<'a, T>
where
    T: ComponentsReader<'a>,
{
    type Item = WasmResult<T>;
    type IntoIter = SectionReaderIntoIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        SectionReaderIntoIter {
            remaining: self.count,
            section: self,
        }
    }
}

struct SectionReaderIntoIter<'a, T> {
    section: SectionReader<'a, T>,
    remaining: usize,
}

impl<'a, T> Iterator for SectionReaderIntoIter<'a, T>
where
    T: ComponentsReader<'a>,
{
    type Item = WasmResult<T>;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.remaining == 0 {
            return None;
        }
        None
    }
}

struct Parser {}

impl<'a> Parser {
    fn parse(reader: &mut BinaryReader<'a>) -> WasmResult<()> {
        let magic = reader
            .ru32()
            .expect(WasmError::UnexpectedMagic.to_string().as_str());
        let version = reader
            .ru32()
            .expect(WasmError::UnexpectedVersion.to_string().as_str());
        // 读取段id
        let sec_id = reader.read_var_u32()? as u8;
        match sec_id {
            SECTION_TYPE => {}
            _ => {}
        }
        Ok(())
    }

    fn section(reader: &mut BinaryReader<'a>) {}
}

pub struct BinaryReader<'a> {
    data: &'a [u8],
    position: usize,
    offset: usize,
    // _marker: marker::PhantomData<T>,
}

struct BinaryIterReader<'a, 'm, T> {
    size: usize,
    reader: &'m BinaryReader<'a>,
    _marker: marker::PhantomData<T>,
}

impl<'a, 'm, T> Iterator for BinaryIterReader<'a, 'm, T> {
    type Item = WasmResult<T>;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        todo!()
    }
}

impl<'a> BinaryReader<'a> {
    fn new_with_offset(data: &'a [u8], offset: usize) {}

    fn new(data: &'a [u8]) -> BinaryReader<'a> {
        Self {
            data: data,
            position: 0,
            offset: data.len(),
        }
    }

    fn eof(&self) -> bool {
        self.position >= self.data.len()
    }

    fn verify_has_byte() -> WasmResult<()> {
        Ok(())
    }

    fn peek(&self) -> WasmResult<u8> {
        Ok(self.data[self.position])
    }

    fn advance(&mut self, size: usize) {
        self.position = self.position + size
    }

    fn read_iter<'m, T>(&'m mut self) -> WasmResult<BinaryIterReader<'a, 'm, T>> {
        let size = self.rusize()?;
        Ok(BinaryIterReader {
            size: size,
            reader: self,
            _marker: marker::PhantomData,
        })
    }

    fn read_var_u32(&mut self) -> WasmResult<u32> {
        let b = self.read_led128_u64::<Binary>()?;
        Ok(b as u32)
    }

    fn ru8(&mut self) -> WasmResult<u8> {
        let b = self.read_u8()?;
        Ok(b)
    }

    //read u32
    fn ru32(&mut self) -> WasmResult<u32> {
        let b = self.read_u32::<LittleEndian>()?;
        Ok(b)
    }

    fn rusize(&mut self) -> WasmResult<usize> {
        let b = self.ru32()?;
        Ok(b as usize)
    }

    fn ru16(&mut self) -> WasmResult<u16> {
        let b = self.read_u16::<LittleEndian>()?;
        Ok(b)
    }
}

impl<'a> Read for BinaryReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = Read::read(&mut &self.data[self.position..], buf)?;
        self.position += n;
        Ok(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 5976746468:[228, 211, 247, 161, 22, 0, 0, 0, 0, 0]
    // 88748464645454:[206, 202, 214, 229, 245, 150, 20, 0, 0, 0]
    // 5789627789625558:[214, 145, 161, 158, 172, 180, 164, 10, 0, 0]
    #[test]
    fn test_leb128() {
        let b = [
            228u8, 211, 247, 161, 22, 206, 202, 214, 229, 245, 150, 20, 214, 145, 161, 158, 172,
            180, 164, 10,
        ];
        let mut r = BinaryReader::new(&b);
        let mut v = r.read_led128_u64::<Binary>().unwrap();
        assert!(5976746468 == v);
        v = r.read_led128_u64::<Binary>().unwrap();
        assert!(88748464645454 == v);
        v = r.read_led128_u64::<Binary>().unwrap();
        assert!(5789627789625558 == v);
    }

    #[test]
    fn test_read() {
        let b = [
            228u8, 211, 247, 161, 22, 206, 202, 214, 229, 245, 150, 20, 214, 145, 161, 158, 172,
            180, 164, 10,
        ];
        let mut r = BinaryReader::new(&b);
        let mut c = [0u8; 5];
        let n = r.read(&mut c).unwrap();
        println!("{},{:?}", n, c);
        let n1 = r.read(&mut c).unwrap();
        println!("{},{:?}", n1, c);

        // r.read_u32()
    }
}
