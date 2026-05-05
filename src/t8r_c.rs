// T8-retest c: String::from_utf8 unwrap on arbitrary bytes.
pub fn decode(bytes: Vec<u8>) -> String {
    String::from_utf8(bytes).unwrap()
}
