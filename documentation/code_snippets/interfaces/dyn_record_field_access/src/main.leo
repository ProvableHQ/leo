struct Metadata {
    inner: u64,
}

program dyn_record_field_access.aleo {
    record Token {
        owner: address,
        balance: u64,
        meta: Metadata,
    }

    // ANCHOR: cast_static_to_dyn
    fn make_dynamic(my_token: Token) -> dyn record {
        let dynamic: dyn record = my_token as dyn record;
        return dynamic;
    }
    // ANCHOR_END: cast_static_to_dyn

    // ANCHOR: check_owner
    fn check_owner(r: dyn record, expected: address) -> bool {
        return r.owner == expected;
    }
    // ANCHOR_END: check_owner

    // ANCHOR: balance_via_let
    fn balance_via_let(r: dyn record) -> u64 {
        let amount: u64 = r.balance;
        return amount;
    }
    // ANCHOR_END: balance_via_let

    // ANCHOR: balance_via_return
    fn balance_via_return(r: dyn record) -> u64 {
        return r.balance;
    }
    // ANCHOR_END: balance_via_return

    // ANCHOR: balance_via_cast
    fn balance_via_cast(r: dyn record) -> u64 {
        return r.balance as u64;
    }
    // ANCHOR_END: balance_via_cast

    // ANCHOR: meta_struct_field
    fn meta(r: dyn record) -> Metadata {
        let m: Metadata = r.meta;
        return m;
    }
    // ANCHOR_END: meta_struct_field

    @noupgrade
    constructor() {}
}
