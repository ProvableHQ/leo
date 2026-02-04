@test
script foo2() {
}

program test.aleo {
    @test
    fn foo1() {
    }

    @test(private_key = "APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH")
    fn foo3() {
    }

    @should_fail
    fn foo4() {
    }

    @should_fail
    @test
    fn foo5() {
    }

    @test(bogus_key = "APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH")
    fn foo6() {
    }
}

@test
@test
script foo7() {
}

@test
script foo8(x: u8) -> u8 {
    return x;
}
