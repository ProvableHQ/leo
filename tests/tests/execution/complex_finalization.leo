/*
namespace: Execute
expectation: Pass
cases:
  - program: four_program.aleo
    function: a
    input: []
*/

// This test checks that the finalization order is correct.
// The functions are invoked in the following order:
// "four::a"
//   --> "two::b"
//        --> "zero::c"
//        --> "one::d"
//   --> "three::e"
//        --> "two::b"
//             --> "zero::c"
//             --> "one::d"
//        --> "one::d"
//        --> "zero::c"
// The future (call graph) produced by the top-level finalize should reflect this structure.

program zero_program.aleo {

  mapping counts: address => u64;

  transition c() {
    return then finalize(self.signer);
  }

  finalize c(addr: address) {
    let count: u64 = counts.get_or_use(addr, 0u64);
    counts.set(addr, count + 1u64);
  }

}


// --- Next Program --- //


program one_program.aleo {

  mapping counts: address => u64;

  transition d() {
    return then finalize(self.signer);
  }

  finalize d(addr: address) {
    let count: u64 = counts.get_or_use(addr, 0u64);
    counts.set(addr, count + 1u64);
  }

}


// --- Next Program --- //


import zero_program.aleo;
import one_program.aleo;

program two_program.aleo {

  mapping counts: address => u64;

  transition b() {
    zero_program.aleo/c();
    one_program.aleo/d();
    return then finalize(self.signer);
  }

  finalize b(addr: address) {
      let count: u64 = counts.get_or_use(addr, 0u64);
      counts.set(addr, count + 1u64);
  }

}


// --- Next Program --- //


import zero_program.aleo;
import one_program.aleo;
import two_program.aleo;

program three_program.aleo {

  mapping counts: address => u64;

  transition e() {
    two_program.aleo/b();
    one_program.aleo/d();
    zero_program.aleo/c();
    return then finalize(self.signer);
  }

  finalize e(addr: address) {
      let count: u64 = counts.get_or_use(addr, 0u64);
      counts.set(addr, count + 1u64);
  }

}


// --- Next Program --- //


import two_program.aleo;
import three_program.aleo;

program four_program.aleo {

  mapping counts: address => u64;

  transition a() {
    two_program.aleo/b();
    three_program.aleo/e();
    return then finalize(self.signer);
  }

  finalize a(addr: address) {
    let count: u64 = counts.get_or_use(addr, 0u64);
    counts.set(addr, count + 1u64);
  }

}
