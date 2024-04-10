/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    mapping amounts: address => u128;

    async transition increase_self(amount: u128) -> Future {
        return finalize_increase_self(self.caller, amount);
    }

    async function finalize_increase_self(addr: address, amount: u128) {
        let current_amount: u128 = Mapping::get_or_use(amounts, addr, 0u128);
        Mapping::set(amounts, addr, current_amount + amount);
    }
}