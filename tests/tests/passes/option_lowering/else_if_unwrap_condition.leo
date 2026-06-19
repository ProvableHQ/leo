program test.aleo {
    @noupgrade
    constructor() {}

    fn main(first: bool, check: u8) -> u8 {
        let missing: u8? = none;

        if first {
            return 1u8;
        } else if missing.unwrap() == check {
            return 2u8;
        } else {
            return 3u8;
        }
    }
}
