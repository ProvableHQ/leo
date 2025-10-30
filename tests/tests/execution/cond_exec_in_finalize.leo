program cond_exec_in_finalize.aleo {

  async transition f(a: u64, b: u64) -> Future {
    return finish(a, b);
  }

  async function finish(a: u64, b: u64) {
    if (b == 0u64) {
      assert_eq(b, 0u64);
    } else {
      assert_eq(a / b, 1u64);
    }
  }

  @noupgrade
  async constructor() {}
}

// --- Test --- //

import cond_exec_in_finalize.aleo;
program test_cond_exec_in_finalize.aleo {

    @test
    async transition f_1() -> Future {
        let f = cond_exec_in_finalize.aleo/f(1u64, 0u64);
        return async {
            f.await();
        };
    }

    @test
    async transition f_2() -> Future {
        let f = cond_exec_in_finalize.aleo/f(1u64, 1u64);
        return async {
            f.await();
        };
    }

    @test
    @should_fail
    async transition f_3() -> Future {
        let f = cond_exec_in_finalize.aleo/f(1u64, 2u64);
        return async {
            f.await();
        };
    }

    @noupgrade
    async constructor() {}
}

