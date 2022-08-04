// This function calculates the interest accrued over ten periods for some `capital` and `rate`.
function fixed_period_interest(capital: u32, public rate: u32) -> u32 {
    let amount: u32 = capital;

    // Accrue for exactly 10 periods.
    for i:u8 in 0u8..10u8 {
        // Note that the added amount is rounded down.
        amount += (amount * rate) / 100u32;
    }
    return amount;
}

// This function calculates the interest accrued over a variable number of periods (max 50) for some `capital` and `rate`.
function bounded_period_interest(capital: u32, public rate: u32, periods: u8) -> u32 {
    console.assert(periods <= 50u8);
    let amount: u32 = capital;

    // Accrue for up to 50 periods.
    for i:u8 in 0u8..50u8 {
        if i < periods {
            // Note that the added amount is rounded down.
            amount += (amount * rate) / 100u32;
        } // Skip the remaining periods.
    }
    return amount;
}
