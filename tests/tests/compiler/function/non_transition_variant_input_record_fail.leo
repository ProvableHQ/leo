
program test.aleo {
    record credits {
        owner: address,
        micrcredits: u64,
    }
}

fn foo(a: credits) -> u8 {
    return 1u8;
}

fn boo(a: address, b: u64) -> credits {
    return credits {
       owner: a,
       micrcredits: b,
   };
}
