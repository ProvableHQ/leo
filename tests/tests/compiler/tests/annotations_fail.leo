
program test.aleo {
    @interpreted_test
    transition foo1() {
    }

    @interpreted_test
    interpret foo2() {
    }

    @native_test(private_key = "APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH")
    transition foo3() {
    }

    @should_fail
    transition foo4() {
    }

    @should_fail
    @native_test
    transition foo5() {
    }

    @native_test(bogus_key = "APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH")
    transition foo6() {
    }

    @interpreted_test
    @interpreted_test
    interpret foo7() {
    }

    @interpreted_test
    interpret foo8(x: u8) -> u8 {
        return x;
    }
}
