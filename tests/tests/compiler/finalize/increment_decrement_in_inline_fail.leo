/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    mapping account: address => u64;
    mapping values: u8 => u8;

    inline foo() {
        increment(values, 0u8, 1u8);
        increment(account, self.caller, 1u64);
    }

    inline bar() {
        decrement(values, 0u8, 1u8);
        decrement(account, self.caller, 1u64);
    }

    finalize finalize_no_params() {
        foo();
        bar();
    }
}
