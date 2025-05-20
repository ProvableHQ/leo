program b28610.aleo {
    record Token {
        owner: address,
    }

    async transition foo() -> Future {
        return bar(self.signer);
    }

    async function bar(addr: address) {
        let t: Token = Token { owner: addr };
        assert(Token { owner: addr }.owner == addr);
    }
}
