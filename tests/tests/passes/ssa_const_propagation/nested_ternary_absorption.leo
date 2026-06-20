program test.aleo {
    @noupgrade
    constructor() {}

    fn absorb_true_branch(cond: bool, a: u8, b: u8) -> u8 {
        let tmp: u8 = cond ? a : b;
        return cond ? tmp : b;
    }

    fn absorb_false_branch(cond: bool, a: u8, b: u8) -> u8 {
        let tmp: u8 = cond ? a : b;
        return cond ? a : tmp;
    }

    fn keep_different_false(cond: bool, a: u8, b: u8, c: u8) -> u8 {
        let tmp: u8 = cond ? a : b;
        return cond ? tmp : c;
    }

    fn keep_different_condition(cond: bool, other: bool, a: u8, b: u8) -> u8 {
        let tmp: u8 = cond ? a : b;
        return other ? tmp : b;
    }
}
