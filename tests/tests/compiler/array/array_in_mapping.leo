
program test.aleo {
    mapping data: address => [bool; 8];

    fn foo(a: [bool; 8]) -> Final {
        let caller = self.caller;
        return final { fin_foo(caller, a); };
    }
}

final fn fin_foo(caller: address, a: [bool; 8]) {
    data.set(caller, a);
}
