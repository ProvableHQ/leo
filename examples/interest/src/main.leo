// The 'interest' main function.
function main(capital: u32, public rate: u32) -> u32 {
    let amount: u32 = capital;
    for i:u8 in 0u8..10u8 { // accrue for 10 periods
        amount += amount * rate / 100u32; // round down
    }
    return amount;
}
