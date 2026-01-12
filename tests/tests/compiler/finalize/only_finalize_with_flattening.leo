struct TokenInfo {
    id: u64,
}

program test.aleo {
    mapping token_name_to_info: field => TokenInfo;


    fn add_new_token () -> Final {
        return final { finalize_add_new_token(); };
    }

    fn add_new_token_2 () -> Final {
        return final { finalize_add_new_token_2(); };
    }
}

final fn finalize_add_new_token() -> () {
    if (false) {
        return;
    }
    if (false) {
        if (true) {
            return;
        }

        let token_info: TokenInfo = TokenInfo {
            id: 0u64,
        };
        Mapping::set(token_name_to_info, 0field, token_info);
    } else {
        return;
    }
}

final fn finalize_add_new_token_2() {
    let try_get_token: TokenInfo = Mapping::get_or_use(
        token_name_to_info,
        0field,
        TokenInfo { id: 0u64 }
    );
    try_get_token = (try_get_token.id == 0u64) ? TokenInfo { id: 10u64 } : try_get_token;
    return;
}
