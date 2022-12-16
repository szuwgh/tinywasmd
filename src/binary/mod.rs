use std::io::Read;

struct Module {
    Magic: u32,
    Version: u32,
    type_section: i32,
}

impl Module {
    fn from_binary(data: &[u8]) {
        if data.len() < 4 {
            panic!("unexpected end of magic header");
        }
    }
}

struct BinaryReader<'a> {
    data: &'a [u8],
    postion: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leb128() {
        //write::Writer::write_sleb128(128);
    }
}
