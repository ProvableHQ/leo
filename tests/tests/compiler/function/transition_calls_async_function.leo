
program test.aleo {

  fn f(a: u64, b: u64) -> Final {
    return final { finish(a, b); };
  }
}

final fn finish(a: u64, b: u64) {
  if (b == 0u64) {
    assert_eq(b, 0u64);
  } else {
    assert_eq(a / b, 1u64);
  }
}
