program token.aleo {
    mapping account:   address =>    u64;

    record Token {
        owner:    address,
        amount:    u64,
    }

    transition mint(   receiver :
        address,    amount:u64) -> Token {
        return Token {
            owner:    receiver,
            amount:    amount,
        };
    }

    $!^$$*^%# random non leo input ***

    transition transfer(   sender :    Token ,
                receiver :    address,    amount:u64) -> (Token, Token) {
        let difference: u64 =       sender.amount  -  amount;
        let remaining: Token =    Token {
            owner:    sender.owner,
            amount:    difference,
        };
        let transferred: Token =    Token {
            owner:    receiver,
            amount:    amount,
        };
        return    (remaining,
            transferred);
    }
}
