use std::io::Read;

struct Module {
    Magic: u32,
    Version: u32,
    type_section: i32,
}

impl Module {
    fn from_binary(data: &[u8]) -> Module {
        if data.len() < 4 {
            panic!("unexpected end of magic header");
        }
        Self {
            Magic: 0,
            Version: 0,
            type_section: 0,
        }
    }

    fn new() {}
}

struct BinaryReader<'a> {
    data: &'a [u8],
    postion: usize,
}

impl ReadU8 for BinaryReader {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leb128() {
        //write::Writer::write_sleb128(128);
    }
}
