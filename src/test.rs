#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("loren ipsum\ndolor sit amet", "lorem", &mut result);
    // Since stdout expects bytes (not strings), we use std::io::Write instead
    // of std::fmt::Write. As a result, we give an empty vector as "writer" in
    // out test (its type will be inferred to Vec<u8>), in the assert_eq! we use a 
    // b"foo". (The b prefix makes this a byte string literal so it type is going
    // to be &[u8] instead of &str).
    assert_eq!(result, b"lorem ipsum\n");
}
