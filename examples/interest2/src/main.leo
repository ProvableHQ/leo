// The 'interest2' main function.
function main(capital: u32, public rate: u32, periods: u8) -> u32 {
    console.assert(periods < 50u8);
    let amount: u32 = capital;
    for i:u8 in 0u8..50u8 { // accrue for max 50 periods
        if i < periods {
            amount += amount * rate / 100u32; // round down
        } // skip remaining periods
    }
    return amount;
}
