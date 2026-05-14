final fn finalize_it() {
    let v: bool = ECDSA::verify_digest([0u8; 65], [0u8; 33], [0u8; 32]);
    assert_eq(v, v);
}

program test.aleo {
    fn main() -> Final {
        return final { finalize_it(); };
    }
}
