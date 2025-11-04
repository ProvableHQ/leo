program test.aleo {
  transition concat_past_array_max(a: [u8; 256], b: [u8; 257]) -> [u8; 32] {
    let c = a + b;
    return c[0..32];
  }
}