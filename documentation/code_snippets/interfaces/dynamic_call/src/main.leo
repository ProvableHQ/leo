interface TokenStandard {
    fn transfer_public(to: address, amount: u64);
}

program dynamic_caller.aleo {
    // ANCHOR: dynamic_call
    // Dynamic: any program that implements TokenStandard can be called
    fn route_transfer_dynamic(token_program: identifier, to: address, amount: u64) {
        return TokenStandard@(token_program)::transfer_public(to, amount);
    }
    // ANCHOR_END: dynamic_call

    fn use_identifier_literal(to: address, amount: u64) {
        // ANCHOR: identifier_literal
        let target: identifier = 'my_program';
        return TokenStandard@(target)::transfer_public(to, amount);
        // ANCHOR_END: identifier_literal
    }

    fn use_identifier_with_network(to: address, amount: u64) {
        // ANCHOR: identifier_with_network
        let target: identifier = 'my_program';
        let net: identifier = 'aleo';
        return TokenStandard@(target, net)::transfer_public(to, amount);
        // ANCHOR_END: identifier_with_network
    }

    @noupgrade
    constructor() {}
}
