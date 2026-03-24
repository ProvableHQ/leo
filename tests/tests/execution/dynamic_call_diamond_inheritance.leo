/*
seed = 123456789

[case]
program = "caller.aleo"
function = "main"
input = ["33560297434230606776804403556field"]
*/

interface Base {
    fn get_value() -> u64;
}

interface Left : Base {
    fn left_op() -> u64;
}

interface Right : Base {
    fn right_op() -> u64;
}

interface Diamond : Left + Right {}

program diamond_impl.aleo : Diamond {
    fn get_value() -> u64 {
        return 42u64;
    }

    fn left_op() -> u64 {
        return 1u64;
    }

    fn right_op() -> u64 {
        return 2u64;
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //

import diamond_impl.aleo;

program caller.aleo {
    fn main(target: field) -> u64 {
        return diamond_impl.aleo::Base@(target)/get_value();
    }

    @noupgrade
    constructor() {}
}
