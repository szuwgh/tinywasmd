struct Module {
    Magic: u32,
    Version: u32,
    type_section: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leb128() {
        write::Writer::write_sleb128(128);
    }
}
