


fn main() {
    let mut file = std::fs::File::create("q").unwrap();
    let mut stdout = std::io::stdout();
    let mut reader: &[u8] = b"hello";
    std::io::copy(&mut reader, &mut file).unwrap();
}