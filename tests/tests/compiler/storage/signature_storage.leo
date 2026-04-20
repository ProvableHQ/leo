program signature_storage.aleo {
    // signature as a plain storage variable
    storage sig: signature;

    // signature inside a fixed-size array
    storage sig_array: [signature; 2];

    // signature inside a vector
    storage sig_vec: [signature];

    fn foo() {}
}
