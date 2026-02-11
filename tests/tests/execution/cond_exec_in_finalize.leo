/*
seed = 123456789
min_height = 16

[case]
program = "cond_exec_in_finalize.aleo"
function = "f"
input = ["1u64", "0u64"]
[case]
program = "cond_exec_in_finalize.aleo"
function = "f"
input = ["1u64", "1u64"]
[case]
program = "cond_exec_in_finalize.aleo"
function = "f"
input = ["1u64", "2u64"]
*/

final fn finish(a: u64, b: u64) {
  if (b == 0u64) {
    assert_eq(b, 0u64);
  } else {
    assert_eq(a / b, 1u64);
  }
}

program cond_exec_in_finalize.aleo {

  fn f(a: u64, b: u64) -> Final {
    return final { finish(a, b); };
  }

  @noupgrade
  constructor() {}
}
