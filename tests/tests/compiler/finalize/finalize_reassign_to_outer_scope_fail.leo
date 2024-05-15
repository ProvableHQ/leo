/*
namespace: Compile
expectation: Fail
configs:
    - dce_enabled : false
*/

program test.aleo {
    struct TokenInfo {
        id: u64,
    }
    mapping token_name_to_info: field => TokenInfo;


    async transition add_new_liquidity_token_2() -> Future {
        return finalize_add_new_liquidity_token_2();
    }


    async function finalize_add_new_liquidity_token_2() {
        let try_get_token: TokenInfo = Mapping::get_or_use(
            token_name_to_info,
            0field,
            TokenInfo { id: 0u64 }
	    );
        if try_get_token.id == 0u64 {
            try_get_token = TokenInfo { id: 10u64 };
        } else {
            return;
        }
    }
}



