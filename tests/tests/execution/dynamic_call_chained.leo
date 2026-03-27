/*
seed = 123456789
min_height = 16

[case]
program = "top_caller.aleo"
function = "main"
input = ["111516215175533field", "1717658988field", "7u32"]
*/

interface Leaf {
    fn process(val: u32) -> (u32, Final);
}

program leaf.aleo: Leaf {
    storage result: u32;

    fn process(val: u32) -> (u32, Final) {
        let doubled: u32 = val * 2u32;
        return (
            doubled,
            final {
                result = doubled;
            },
        );
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //
import leaf.aleo;

interface Middle {
    fn relay(leaf_target: field, val: u32) -> (u32, Final);
}

program middle.aleo: Middle {
    fn relay(leaf_target: field, val: u32) -> (u32, Final) {
        let (result, f): (u32, Final) = leaf.aleo::Leaf@(leaf_target)/process(val);
        return (
            result,
            final {
                f.run();
            },
        );
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //
import leaf.aleo;
import middle.aleo;

program top_caller.aleo {
    fn main(middle_target: field, leaf_target: field, val: u32) -> (u32, Final) {
        let (result, f): (u32, Final) = middle.aleo::Middle@(middle_target)/relay(leaf_target, val);
        return (
            result,
            final {
                f.run();
            },
        );
    }

    @noupgrade
    constructor() {}
}
