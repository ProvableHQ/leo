program test.aleo {
    fn t1() {
        let f1 = final {
            let s = std::ctx::signer();
            let c = std::ctx::caller();
        };
    }

    @noupgrade
    constructor() {}
}
