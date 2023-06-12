/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    mapping account: address => u64;
    mapping values: u8 => u8;

    inline foo() {
        Mapping::set(values, 0u8, 1u8);
        Mapping::get_or_use(account, self.caller, 1u64);
        Mapping::get(values, 1u8);
    }

    inline bar() {
        Mapping::set(values, 0u8, 1u8);
        Mapping::get_or_use(account, self.caller, 1u64);
        Mapping::get(values, 0u8);
    }

    finalize finalize_no_params() {
        foo();
        bar();
    }
}
