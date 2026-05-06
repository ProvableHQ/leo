// ANCHOR: program
import token.aleo;

program swap.aleo {
    fn do_swap(owner: address, amount: u32) -> u32 {
        return token.aleo::mint(owner, amount);
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: program
