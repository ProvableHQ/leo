program test.aleo {
    fn bar() {
        let x: u32 = std::ctx::caller();
        let y: u8 = std::ctx::caller();

        let z: u8 = 0u8;
        z = std::ctx::caller();

        let w: field = 0field;
        w = std::ctx::signer();
    }

    @noupgrade
    constructor() {}
}
