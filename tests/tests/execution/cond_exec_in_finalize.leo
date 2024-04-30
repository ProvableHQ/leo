/*
namespace: Execute
expectation: Pass
cases:
  - program: cond_exec_in_finalize.aleo
    function: a
    input: ["1u64", "0u64"]
  - program: cond_exec_in_finalize.aleo
    function: a
    input: ["1u64", "1u64"]
  - program: cond_exec_in_finalize.aleo
    function: a
    input: ["1u64", "2u64"]
*/


program cond_exec_in_finalize.aleo {

  async transition a(a: u64, b: u64) -> Future {
    return finish(a, b);
  }

  async function finish(a: u64, b: u64) {
    if (b == 0u64) {
      assert_eq(b, 0u64);
    } else {
      assert_eq(a / b, 1u64);
    }
  }
}
