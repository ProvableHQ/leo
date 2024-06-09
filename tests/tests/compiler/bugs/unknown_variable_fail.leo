/*
namespace: Compile
expectation: Fail
*/

program test.aleo {

    struct TokenInfo {
        name: field,
        symbol: field,
        decimals: u8,
        circulating_supply: u64,
        total_supply: u64,
        testers: u64
        admin: address,
    }


    struct TestToken {
        name: field,
        symbol: field,
        decimals: u8,
        circulating_supply: u64,
        total_supply: u64,
        admin: address,
    }

    struct GlobalState {
        next_token_id: field,
        admin: address,
    }

    mapping tokens: u64 => TokenInfo;

    mapping test_token: u64 => TestToken;

    mapping global_state: bool => GlobalState;

    async transition init(b: bool) -> (address, Future) {
        return (self.caller, finalize_init(caller));
    }

    async function finalize_init(caller: address) {
        let gs: GlobalState = Mapping::get_or_use(global_state, true, GlobalState {
            next_token_id: 0field,
            admin: aleo1az8p9vlllyqwtj0c2g9svkd0e5v0p3zzdflwwrpa7kpe8xrfxgfqqpru7m,
        });
        assert_eq(gs.next_token_id, 0field);
        assert_eq(caller, gs.admin);
        Mapping::set(global_state, true, GlobalState {
            next_token_id: 1field,
            admin: gs.admin,
        });

        let test_supply: u64 = 15000000000000u64;
        let admin_token_amt: u64 = 60000000u64;
        let owner_amount: u64 = Mapping::get_or_use(account, caller, 0u64);
        assert(owner_amount == 0u64);
        Mapping::set(account, caller, owner_amount + admin_token_amt);

        Mapping::set(test_token, test_supply, TestToken {
            name: 4577111110111112111108121field,
            symbol: 4577111110111112111108121field,
            decimals: 6u8,
            circulating_supply: 0u64 + admin_token_amt,
            total_supply: test_supply,
            admin: gs.admin,
        });
    }

}


