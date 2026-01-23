program b28610.aleo {
    record Token {
        owner: address,
    }

    fn foo() -> Final {
        return bar(self.signer);
    }
}

onchain fn bar(addr: address) {
    let t: Token = Token { owner: addr };
    assert(Token { owner: addr }.owner == addr);
}
