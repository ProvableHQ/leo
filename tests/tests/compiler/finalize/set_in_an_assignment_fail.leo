/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    mapping amounts: address => u128;

    async transition decrease_self(amount: u128) -> Future {
        return finalize_decrease_self(self.caller, amount);
    }
    
    async function finalize_decrease_self(addr: address, amount: u128) {
        let result: () = Mapping::set(amounts, addr, amount);
        let result: u128 = Mapping::set(amounts, addr, amount);
    }
}
