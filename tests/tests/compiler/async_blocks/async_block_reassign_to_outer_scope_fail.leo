struct TokenInfo {
    id: u64,
}

program test.aleo {
    mapping token_name_to_info: field => TokenInfo;

    fn add_new_token_2() -> Final {
        return final {
            let try_get_token: TokenInfo = Mapping::get_or_use(
                token_name_to_info,
                0field,
                TokenInfo { id: 0u64 }
            );
            if try_get_token.id == 0u64 {
                try_get_token = TokenInfo { id: 10u64 };
            }
        };
    }
}
