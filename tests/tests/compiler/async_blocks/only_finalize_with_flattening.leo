
program test.aleo {
    struct TokenInfo {
        id: u64,
    }
    mapping token_name_to_info: field => TokenInfo;

    async transition add_new_token () -> Future {
        return async {
            if false {
                let token_info: TokenInfo = TokenInfo {
                    id: 0u64,
                };
                Mapping::set(token_name_to_info, 0field, token_info);
            }
        };
    }

    async transition add_new_token_2 () -> Future {
        return async {
            let try_get_token: TokenInfo = Mapping::get_or_use(
                token_name_to_info,
                0field,
                TokenInfo { id: 0u64 }
            );
            try_get_token = (try_get_token.id == 0u64) ? TokenInfo { id: 10u64 } : try_get_token;
        };
    }
}
