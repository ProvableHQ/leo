
struct TokenInfo {
    id: u64,
}

program test.aleo {
    fn add_new_liquidity_token_2() {
        let try_get_token: TokenInfo = TokenInfo { id: 0u64 };
        if try_get_token.id == 0u64 {
            try_get_token = TokenInfo { id: 10u64 };
        } else {
            return;
        }
    }
}
