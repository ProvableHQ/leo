program b28610.aleo {
    record Token {
        owner: address,
    }

    fn foo() -> Final {
        let signer = self.signer;
        return final { bar(signer); };
    }
}

final fn bar(addr: address) {
    let t: Token = Token { owner: addr };
    assert(Token { owner: addr }.owner == addr);
}
