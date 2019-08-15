use std::error::Error;
use std::fmt::Write;

fn is_ascii_printable(c: &u8) -> bool {
    *c >= 0x20 && *c < 0x7f
}

pub struct HexDump<'a> {
    buffer: &'a Vec<u8>,
}

impl<'a> HexDump<'a> {
    pub fn new(buffer: &'a Vec<u8>) -> Self {
        HexDump { buffer }
    }

    pub fn dump(&self) -> Result<(), Box<Error>> {
        let mut buf = String::with_capacity(64);
        let chunks = self.buffer.chunks(16);
        let chunks_length = chunks.len();

        for (i, chunk16) in chunks.enumerate() {
            print!("{:08x}", i * 16); // byte index
            buf.clear();

            for chunk8 in chunk16.chunks(8) {
                write!(&mut buf, " ")?;
                for i in chunk8 {
                    write!(&mut buf, " {:02x}", i)?;
                }
            }

            print!("{:<50}", buf);

            buf.clear();
            for c in chunk16 {
                let out_char = if is_ascii_printable(c) {
                    *c as char
                } else {
                    '.'
                };
                write!(&mut buf, "{}", out_char as char)?;
            }
            println!("  |{:<16}|", buf);
        }
        println!("{:08x}", chunks_length * 16);

        Ok(())
    }
}
