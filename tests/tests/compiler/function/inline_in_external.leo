// This test ensures that `FunctionInlining` works in Leo stubs

program child.aleo {
    async transition foo() -> Future {
      return async {
        assert(f() == 4);
        assert(f() != 4);
      };
    }

    inline f() -> u32 { return 4; }
}

// --- Next Program --- //

import child.aleo;
program parent.aleo {
    async transition foo() -> Future {
      return async {
        assert(f() == 4);
      };
    }

    inline f() -> u32 { return 4; }
}
