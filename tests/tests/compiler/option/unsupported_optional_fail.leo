program invalid_optional_types.aleo {
    // Tuples cannot be optional
    fn tuple_optional_type() {
        let a: (u8, u8)? = (1u8, 2u8); // ERROR: Tuples cannot be optional just yet
    }

    fn sig_or_address_type(s_: signature, a_: address) {
        let s: signature? = s_;
        let a: address? = a_;
    }

    // Final cannot be optional
    fn future_optional_type() {
        let fut: Final? = final { finalize(); }; // ERROR: Final? is not allowed
    }
    @noupgrade
    constructor() {}
}

// Dummy future-returning function
final fn finalize() {
}

