    // Nested struct with optional field
    struct InnerStruct {
        val: u64?,
    }

program record_with_optional_fields.aleo {
    record BadRecord1 {
        owner: address,
        data: u8?, // ERROR: optional type directly
    }

    // Array containing optionals — not allowed
    record BadRecord2 {
        owner: address,
        flags: [bool?; 4], // ERROR: contains optional
    }

    record BadRecord3 {
        owner: address,
        inner: InnerStruct, // ERROR: contains optional via nested struct
    }

    fn foo() {}

    @noupgrade
    constructor() {}
}
