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

final fn finalize_c(addr: address) {
  let count: u64 = counts.get_or_use(addr, 0u64);
  counts.set(addr, count + 1u64);
}

program zero_program.aleo {

  mapping counts: address => u64;

  fn c() -> Final {
    let signer = self.signer;
    return final { finalize_c(signer); };
  }

  @noupgrade
  constructor() {}

}


// --- Next Program --- //

final fn finalize_d(addr: address) {
  let count: u64 = counts.get_or_use(addr, 0u64);
  counts.set(addr, count + 1u64);
}

program one_program.aleo {

  mapping counts: address => u64;

  fn d() -> Final {
    let signer = self.signer;
    return final { finalize_d(signer); };
  }

  @noupgrade
  constructor() {}

}


// --- Next Program --- //


import zero_program.aleo;
import one_program.aleo;

final fn finalize_b(f0: Final, f1: Final, addr: address) {
    f0.run();
    f1.run();
    let count: u64 = counts.get_or_use(addr, 0u64);
    counts.set(addr, count + 1u64);
}

program two_program.aleo {

  mapping counts: address => u64;

  fn b() -> Final {
    let f0: Final = zero_program.aleo/c();
    let f1: Final = one_program.aleo/d();
    let signer = self.signer;
    return final { finalize_b(f0, f1, signer); };
  }

  @noupgrade
  constructor() {}

}


// --- Next Program --- //


import zero_program.aleo;
import one_program.aleo;
import two_program.aleo;

final fn finalize_e(f0: Final, f1: Final, f2: Final, addr: address) {
    f0.run();
    f1.run();
    f2.run();
    let count: u64 = counts.get_or_use(addr, 0u64);
    counts.set(addr, count + 1u64);
}

program three_program.aleo {

  mapping counts: address => u64;

  fn e() -> Final {
    let f0: Final = two_program.aleo/b();
    let f1: Final = one_program.aleo/d();
    let f2: Final = zero_program.aleo/c();
    let signer = self.signer;
    return final { finalize_e(f0, f1, f2, signer); };
  }

  @noupgrade
  constructor() {}

}


// --- Next Program --- //


import two_program.aleo;
import three_program.aleo;

final fn finalize_a(f0: Final, f1: Final, addr: address) {
  f0.run();
  f1.run();
  let count: u64 = counts.get_or_use(addr, 0u64);
  counts.set(addr, count + 1u64);
}

program four_program.aleo {

  mapping counts: address => u64;

  fn a() -> Final {
    let f0: Final = two_program.aleo/b();
    let f1: Final = three_program.aleo/e();
    let signer = self.signer;
    return final { finalize_a(f0, f1, signer); };
  }

  @noupgrade
  constructor() {}

}
