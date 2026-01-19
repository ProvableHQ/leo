
program test.aleo {
    mapping data: address => [bool; 8];

    async fn foo(a: [bool; 8]) -> Future {
        return fin_foo(self.caller, a);
    }
}

async fn fin_foo(caller: address, a: [bool; 8]) {
    data.set(caller, a);
}
