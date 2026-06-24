program b28610.aleo {
    record Token {
        owner: address,
    }

    fn foo() -> Final {
        let signer = std::ctx::signer();
        return final { bar(signer); };
    }

    @noupgrade
    constructor() {}
}

final fn bar(addr: address) {
    let t: Token = Token { owner: addr };
    assert(Token { owner: addr }.owner == addr);
}
