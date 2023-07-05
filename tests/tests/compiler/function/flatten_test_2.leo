/*
namespace: Compile
expectation: Pass
configs:
    - dce_enabled : true
    - dce_enabled : false
*/

program test.aleo {
    struct TokenInfo {
        id: u64,
    }

    transition add_new_liquidity_token_2() {
        let try_get_token: TokenInfo = TokenInfo { id: 0u64 };
        if try_get_token.id == 0u64 {
            try_get_token = TokenInfo { id: 10u64 };
        } else {
            return;
        }
    }
}
