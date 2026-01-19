program b28610.aleo {
    record Token {
        owner: address,
    }

    async fn foo() -> Future {
        return bar(self.signer);
    }
}

async fn bar(addr: address) {
    let t: Token = Token { owner: addr };
    assert(Token { owner: addr }.owner == addr);
}
