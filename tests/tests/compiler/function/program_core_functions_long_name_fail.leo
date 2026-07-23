
export final fn foo() {
    let c: [u8; 32] = std::prog::checksum::[aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.aleo]();
    let d: u16 = std::prog::edition::[aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.aleo]();
    let e: address = std::prog::program_owner::[aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.aleo]();
}

program test.aleo {
    fn bar() -> Final {
        return final { foo(); };
    }

    @noupgrade
    constructor() {}
}
