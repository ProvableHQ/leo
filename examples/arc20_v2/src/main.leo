// The 'arc20_v2' program.
import arc20.leo;

program arc20_v2.aleo {
    record CredentialV2 {
        owner: address,
        token_id: u64,
        total_supply: u64,
        // Minted amount 
        supplyed: u64,
    }

    record TokenV2 {
        owner: address,
        token_id: u64,
        amount: u64,
    }

    struct OrderV2 {
        caller: address,
        hold_token_id: u64,
        hold_token_amount: u64,

        expect_token_id: u64,
        expect_token_amount: u64,
    }
    struct PairV2{
        addr: address,
        token_id: u64,
    }
    mapping existV2: u64 => bool;
    mapping balancesV2: PairV2 => u64;

    // Destroy the token_v1 and create token_v2
    transition credentialv1_for_credentialv2(c1: arc20.leo/Credential.record) -> CredentialV2 {
        let c2: CredentialV2 = CredentialV2 {
            owner: c1.owner,
            token_id: c1.token_id,
            total_supply: c1.total_supply,
            supplyed: c1.supplyed,
        };
        arc20.leo/drop(c1);
        return c2;
    }
    transition tokenv1_for_tokenv2(t1: arc20.leo/Token.record) -> TokenV2{
        let t2: TokenV2 = TokenV2 {
            owner: t1.owner,
            token_id: t1.token_id,
            amount: t1.amount,
        };
        arc20.leo/burn(t1);
        return t2;   
    }

    // transfer 
    // mint 
}
