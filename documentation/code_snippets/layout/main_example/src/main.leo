// ANCHOR: file
import foo.aleo;

const FOO: u64 = 1u64;

struct Message {
    sender: address,
    object: u64,
}

fn compute(a: u64, b: u64) -> u64 {
    return a + b + FOO;
}

program hello.aleo {
    mapping account: address => u64;

    record Token {
        owner: address,
        amount: u64,
    }

    fn mint_public(
        public receiver: address,
        public amount: u64,
    ) -> (Token, Final) {
        let token: Token = Token { owner: receiver, amount };
        return (token, final {
            let current_amount: u64 = Mapping::get_or_use(account, receiver, 0u64);
            Mapping::set(account, receiver, current_amount + amount);
        });
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: file
