/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    mapping balances: address => u128;
    
    function read(public addr: address) -> public u128 {
        return balances[addr];
    }
    
    function write(public addr: address, public amount: u128) {
        balances[addr] = amount;
    }
    
    function read_in_finalize(public addr: address) {
        finalize(addr);
    }
    
    finalize read_in_finalize(public addr: address) -> public u128 {
        return balances[addr];
    }
    
    function write_in_finalize(public addr: address, public amount: u128) {
        return then finalize(addr, amount);
    }
    
    finalize write_in_finalize(public: addr: address, public amount: u128) {
        balances[addr] = amount;
    }
}
