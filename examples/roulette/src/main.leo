// The 'roulette' main function.
record Token {
    owner: address,
    gates: u64,
    amount: u64
}

// Returns a new `Token` record that represents the casino address and amount of funds.
@program
function mint_casino_token_record(a: address, b: u64) -> Token {
    return Token {
        owner: a,
        gates: 0u64,
        amount: b,
    };
}

// Returns the updated casino and player records after the result of the bet.
@program
function make_bet(
    // Casino token record
    casino: Token,
    // Player making a bet
    player: address,
    // Random roulette result
    spin: u8,
    // Player bet number
    bet: u8,
    // Player bet amount of tokens
    bet_amount: u64,
    // Player balance of available tokens
    balance: u64,
) -> (Token, Token) {
    let win: bool = spin == bet;

    let reward: u64 = bet_amount * 35u64;

    let casino_balance: u64 = 0u64;
    let player_balance: u64 = 0u64;

    if win {
        casino_balance = casino.amount - reward;
        player_balance = balance + reward;
    } else {
        casino_balance = casino.amount + reward;
        player_balance = balance - reward;
    }

    let casino_output: Token = Token {
        owner: casino.owner,
        gates: casino.gates,
        amount: casino_balance,
    };

    let player_output: Token = Token {
        owner: player,
        gates: 0u64,
        amount: player_balance,
    };

    return (casino_output, player_output);
}

// Returns the output of hashing `a` with the poseidon hash function.
@program
function psd_hash(public a: u32) -> field {
    return Poseidon2::hash(a);
}

// Checks if the six bits `r0 - r5` match the u16 value at `r6`
@program
function psd_bits_mod(
    public r0: bool, // Most significant bit
    public r1: bool,
    public r2: bool,
    public r3: bool,
    public r4: bool,
    public r5: bool,
    public r6: u16,
) -> bool {
    let r7: u16 = r5 ? 1u16 : 0u16;
    let r8: u16 = r4 ? 2u16 : 0u16;
    let r9: u16 = r3 ? 4u16 : 0u16;
    let r10: u16 = r2 ? 8u16 : 0u16;
    let r11: u16 = r1 ? 16u16 : 0u16;
    let r12: u16 = r0 ? 32u16 : 0u16;

    let sum: u16 = r7 + r8 + r9 + r10 + r11 + r12;
    let result: u16 = sum % 36u16;

    return r6 == result;
}
