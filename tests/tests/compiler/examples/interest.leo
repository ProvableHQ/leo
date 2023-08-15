/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    // This function calculates the interest accrued
    // over ten iterations for some `capital` and `rate`.
    transition fixed_iteration_interest(capital: u32, public rate: u32) -> u32 {
        let amount: u32 = capital;

        // Accrue for exactly 10 iterations.
        for i:u8 in 0u8..10u8 {
            // Note that the added amount is rounded down.
            amount += (amount * rate) / 100u32;
        }
        return amount;
    }


    // This function calculates the interest accrued
    // over a variable number of iterations (max 50) for some `capital` and `rate`.
    transition bounded_iteration_interest(capital: u32,
                                        public rate: u32,
                                        iterations: u8) -> u32 {
        assert(iterations <= 50u8);
        let amount: u32 = capital;

        // Accrue for up to 50 iterations.
        for i:u8 in 0u8..50u8 {
            if i < iterations {
                // Note that the added amount is rounded down.
                amount += (amount * rate) / 100u32;

            } // Skip the remaining iterations.
            if i == 40u8 {
                return amount;
            }
        }
        return amount;
    }
}
