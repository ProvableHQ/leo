program invalid_optional_types.aleo {
    // Tuples cannot be optional
    transition tuple_optional_type() {
        let a: (u8, u8)? = (1u8, 2u8); // ERROR: Tuples cannot be optional just yet
    }

    transition sig_or_address_type(s_: signature, a_: address) {
        let s: signature? = s_;
        let a: address? = a_;
    }

    // Future cannot be optional
    async transition future_optional_type() {
        let fut: Future? = finalize(); // ERROR: Future? is not allowed
    }

    // Dummy future-returning function
    async function finalize() {
    }

    @noupgrade
    async constructor() {}
}
