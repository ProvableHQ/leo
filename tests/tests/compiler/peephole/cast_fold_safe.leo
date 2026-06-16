program test.aleo {
    fn main(x: u32) -> u16 {
        let wide: u64 = x as u64;
        let narrow: u16 = wide as u16;
        return narrow;
    }

    @noupgrade
    constructor() {}
}
