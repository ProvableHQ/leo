/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    mapping amounts: address => u128;

    transition increase_self(amount: u128) {
        return then finalize(self.caller, amount);
    }

    finalize increase_self(addr: address, amount: u128) {
        let current_amount: u128 = Mapping::get_or_use(amounts, addr, 0u128);
        Mapping::set(amounts, addr, current_amount + amount);
    }
}