/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    function calculate_interest(principal: u64, rate: u64, periods: u64) -> u64 {
        let amount: u64 = principal;

        for i:u64 in 0u64..1000000000000000000000000000000000000000000000000000000000000000000000000000000000000u64 {
            if i < periods {
                amount += (amount * rate) / 10000u64;
            }
        }
        return amount;
    }
}