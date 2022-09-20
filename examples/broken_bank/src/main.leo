record Token {
    owner: address,
    gates: u64,
    amount: u64,
}

mapping balances: field => u64;

@program
function issue(owner: address, amount: u64) -> Token {
    console.assert_eq(self.caller, aleo1t0uer3jgtsgmx5tq6x6f9ecu8tr57rzzfnc2dgmcqldceal0ls9qf6st7a);
    return Token {
        owner: owner,
        gates: 0u64,
        amount: amount,
    };
}

@program
function deposit(token: Token, amount: u64) -> Token {
    let difference: u64 = token.amount - amount;

    let remaining: Token = Token {
        owner: token.owner,
        gates: token.gates,
        amount: difference,
    };

    let hash: field = BHP256::hash(token.owner);

    finalize(hash, amount);

    return remaining;
}

finalize deposit(hash: field, amount: u64) {
    increment(balances, hash, amount);
}

@program
function withdraw(recipient: address, amount: u64, rate: u64, periods: u64) -> Token {
    console.assert_eq(self.caller, aleo1t0uer3jgtsgmx5tq6x6f9ecu8tr57rzzfnc2dgmcqldceal0ls9qf6st7a);
    let hash: field = BHP256::hash(recipient);

    let total: u64 = calculate_interest(amount, rate, periods);

    let token: Token = Token {
        owner: recipient,
        gates: 0u64,
        amount: total,
    };

    finalize(hash, amount);

    return token;
}

finalize withdraw(hash: field, amount: u64) {
    decrement(balances, hash, amount);
}

function calculate_interest(principal: u64, rate: u64, periods: u64) -> u64 {
    let amount: u64 = principal;

    for i:u64 in 0u64..100u64 {
        if i < periods {
            amount += (amount * rate) / 10000u64;
        }
    }

    return amount;
}



