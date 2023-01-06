use crate::util::error::{WasmError, WasmResult};
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Read;
use varintrs::{Binary, ReadBytesVarExt};

const SECTION_CUSTOM: u8 = 0;
const SECTION_TYPE: u8 = 1;

struct Module {
    Magic: u32,
    Version: u32,
    type_section: Box<[FunctionType]>,
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

pub enum Type {
    Func(FunctionType),
}

struct FunctionType {
    params: Box<[ValType]>,
    returns: Box<[ValType]>,
}

impl<'a> FunctionType {
    fn from_reader(reader: &mut BinaryReader<'a>) -> WasmResult<FunctionType> {
        let func_type_count = reader.read_var_u32()?;
        
        Ok(())
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

struct Parser {}

impl<'a> Parser {
    fn parse(reader: &mut BinaryReader<'a>) -> WasmResult<()> {
        let magic = reader
            .ru32()
            .expect(WasmError::UnexpectedMagic.to_string().as_str());
        let version = reader
            .ru32()
            .expect(WasmError::UnexpectedVersion.to_string().as_str());

        let sec_id = reader.read_var_u32()?;
        Ok(())
    }
}

struct BinaryReader<'a> {
    data: &'a [u8],
    postion: usize,
}

impl<'a> BinaryReader<'a> {
    fn new(data: &'a [u8]) -> BinaryReader<'a> {
        Self {
            data: data,
            postion: 0,
        }
    }

    fn read_var_u32(&mut self) -> WasmResult<u32> {
        let b = self.read_led128_u64::<Binary>()?;
        Ok(b as u32)
    }

    fn ru8(&mut self) -> WasmResult<u8> {
        let b = self.read_u8()?;
        Ok(b)
    }

    fn ru32(&mut self) -> WasmResult<u32> {
        let b = self.read_u32::<LittleEndian>()?;
        Ok(b)
    }

    fn ru16(&mut self) -> WasmResult<u16> {
        let b = self.read_u16::<LittleEndian>()?;
        Ok(b)
    }
}

impl<'a> Read for BinaryReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = Read::read(&mut &self.data[self.postion..], buf)?;
        self.postion += n;
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
