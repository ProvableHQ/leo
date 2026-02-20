program test0.aleo {
    record Token {
        owner: address,
    }

    fn foo() -> Token {
        return Token {
            owner: self.caller,
        };
    }

    fn bar() -> Token {
        let t: Token = Token {
            owner: self.caller,
        };

        return t;
    }
}
