/*
seed = 123456789
min_height = 16

[case]
program = "four_program.aleo"
function = "a"
input = []
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

onchain fn finalize_c(addr: address) {
  let count: u64 = counts.get_or_use(addr, 0u64);
  counts.set(addr, count + 1u64);
}

program zero_program.aleo {

  mapping counts: address => u64;

  fn c() -> Future {
    return finalize_c(self.signer);
  }

  @noupgrade
  constructor() {}

}


// --- Next Program --- //

onchain fn finalize_d(addr: address) {
  let count: u64 = counts.get_or_use(addr, 0u64);
  counts.set(addr, count + 1u64);
}

program one_program.aleo {

  mapping counts: address => u64;

  fn d() -> Future {
    return finalize_d(self.signer);
  }

  @noupgrade
  constructor() {}

}


// --- Next Program --- //


import zero_program.aleo;
import one_program.aleo;

onchain fn finalize_b(f0: Future, f1: Future, addr: address) {
    f0.await();
    f1.await();
    let count: u64 = counts.get_or_use(addr, 0u64);
    counts.set(addr, count + 1u64);
}

program two_program.aleo {

  mapping counts: address => u64;

  fn b() -> Future {
    let f0: Future = zero_program.aleo/c();
    let f1: Future = one_program.aleo/d();
    return finalize_b(f0, f1, self.signer);
  }

  @noupgrade
  constructor() {}

}


// --- Next Program --- //


import zero_program.aleo;
import one_program.aleo;
import two_program.aleo;

onchain fn finalize_e(f0: Future, f1: Future, f2: Future, addr: address) {
    f0.await();
    f1.await();
    f2.await();
    let count: u64 = counts.get_or_use(addr, 0u64);
    counts.set(addr, count + 1u64);
}

program three_program.aleo {

  mapping counts: address => u64;

  fn e() -> Future {
    let f0: Future = two_program.aleo/b();
    let f1: Future = one_program.aleo/d();
    let f2: Future = zero_program.aleo/c();
    return finalize_e(f0, f1, f2, self.signer);
  }

  @noupgrade
  constructor() {}

}


// --- Next Program --- //


import two_program.aleo;
import three_program.aleo;

onchain fn finalize_a(f0: Future, f1: Future, addr: address) {
  f0.await();
  f1.await();
  let count: u64 = counts.get_or_use(addr, 0u64);
  counts.set(addr, count + 1u64);
}

program four_program.aleo {

  mapping counts: address => u64;

  fn a() -> Future {
    let f0: Future = two_program.aleo/b();
    let f1: Future = three_program.aleo/e();
    return finalize_a(f0, f1, self.signer);
  }

  @noupgrade
  constructor() {}

}
