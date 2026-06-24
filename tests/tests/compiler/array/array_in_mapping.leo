program test.aleo {
    mapping data: address => [bool; 8];

    fn foo(a: [bool; 8]) -> Final {
        let caller = std::ctx::caller();
        return final { fin_foo(caller, a); };
    }

    @noupgrade
    constructor() {}
}

final fn fin_foo(caller: address, a: [bool; 8]) {
    data.set(caller, a);
}
