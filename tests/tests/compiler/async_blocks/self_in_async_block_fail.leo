program test.aleo {
    fn t1() {
        let f1 = final {
            let c = std::ctx::caller();
            let s = std::ctx::signer();
        };
    }

    @noupgrade
    constructor() {}
}
