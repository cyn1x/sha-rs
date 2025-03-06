mod sha1;

pub fn sha1(input: &[u8]) -> [u8; 20] {
    let res = sha1::x86::compress(input);
    res
}
