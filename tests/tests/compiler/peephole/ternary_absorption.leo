program test.aleo {
    fn true_branch(cond: bool, a: u32, b: u32) -> u32 {
        return cond ? (cond ? a : b) : b;
    }

    fn false_branch(cond: bool, a: u32, b: u32) -> u32 {
        return cond ? a : (cond ? a : b);
    }

    fn different_condition(cond: bool, other: bool, a: u32, b: u32) -> u32 {
        return cond ? (other ? a : b) : b;
    }

    @noupgrade
    constructor() {}
}
