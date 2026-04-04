// Test: negative case for the Type::Tuple branch in type_eq_with_record_names.
// The interface requires split() to return (Token, Token), but the program returns
// (Token, u64). The second tuple element differs, so signature_mismatch must fire.

interface ISplit {
    record Token {
        owner: address,
        ..
    }
    fn split(t: Token) -> (Token, Token);
}

program test.aleo: ISplit {
    record Token {
        owner: address,
        amount: u64,
    }

    fn split(t: Token) -> (Token, u64) {
        return (t, t.amount);
    }
}
