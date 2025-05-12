program test0.aleo {
    record Token {
        owner: address,
    }

    transition foo() -> Token {
        return Token {
            owner: self.caller,
        };
    }

    transition bar() -> Token {
        let t: Token = Token {
            owner: self.caller,
        };

        return t;
    }
}
