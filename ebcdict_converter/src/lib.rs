mod byte_string;
use byte_string::ByteString;

// Converts a hex ebcdic string to utf-16 plaintext string.
// because ebcdic chars are 2 bytes long we can assume even length
// and step of 2 for each character
pub fn ebcdic_hex_to_ascii(hex_string: String) -> String {

    // removing whitespace from the string if any
    let raw_string: String = hex_string.chars().filter(|c| !c.is_whitespace()).collect();
    let ebcic_bytes = ByteString::from(raw_string.as_str());

    ebcic_bytes.to_string()
}