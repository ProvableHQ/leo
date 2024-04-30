/*
namespace: Compile
expectation: Fail
*/

program test.aleo {

  transition a(a: u64, b: u64) -> Future {
    return finish(a, b);
  }

  async transition finish(a: u64, b: u64) {
    if (b == 0u64) {
      assert_eq(b, 0u64);
    } else {
      assert_eq(a / b, 1u64);
    }
  }
}
