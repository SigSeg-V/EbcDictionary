mod ebcdic_table;
use ebcdic_table::EBCDIC_TO_ASCII;
pub struct ByteString {
    hex_arr: Vec<String>,
}

impl From<&str> for ByteString {
    fn from(value: &str) -> Self {
        // check for unviable strings where there is a half-byte somewhere
        if value.len().rem_euclid(2) != 0 {
            panic!("Invalid hex string, odd string length")
        }
        
        // from here our string can be safely put into a bytestring
        let mut hex_vec: Vec<String> = vec![];

        // converted from any case to upper for matching to table
        for i in (0..value.len()).step_by(2) {
            hex_vec.push(value[i..i+2].to_string().to_uppercase())
        }

        ByteString {
            hex_arr: hex_vec
        }
    }
}

impl ByteString {
    pub fn to_string(&self) -> String {
        let mut res: String = "".to_string();
        for byte in self.hex_arr.iter() {
            if EBCDIC_TO_ASCII.contains_key(&byte) {
                res.push(EBCDIC_TO_ASCII[&byte]);
            }
            else {
                println!("Warning: unknown token {byte}");

                // assume the missing char is a space for spacing issues with non-zero width chars
                res.push(' ');
            }
        }

        res
    }
}
