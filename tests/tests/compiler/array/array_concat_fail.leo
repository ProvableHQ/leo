program test.aleo {
  transition concat_past_array_max() -> [u8; 32] {
    let a = [0u8; 512];
    let b = [1u8; 512];
    let c = a + b; 
    return c[16..48];
  }
}