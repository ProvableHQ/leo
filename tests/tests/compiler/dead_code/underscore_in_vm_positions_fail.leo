// Every identifier position whose name is emitted verbatim to the Aleo bytecode
// is rejected by `NameValidation` if it starts with `_`. snarkVM requires
// identifiers to start with a letter, so a leading `_` would otherwise only
// surface as a deploy-time failure.
//
// Companion positive cases for the *allowed* `_` positions (let, const,
// loop iter, free fn name, final fn name, parameter, const-generic) live in
// `unused_silenced.leo`.

// Top-level struct name and struct member name.
struct _BadStruct {
    _bad_struct_field: u32,
    ok_field: u32,
}

program _bad_program.aleo {
    // Record name and record member name.
    record _BadRecord {
        owner: address,
        _bad_record_field: u32,
    }

    // Mapping name.
    mapping _bad_mapping: address => u128;

    // Storage variable name.
    storage _bad_storage: u32;

    // Entry-point function name.
    fn _bad_entry() -> u32 {
        return 1u32;
    }

    // View function name. Views are externally callable and emitted verbatim
    // as `function <name>:`, so a leading `_` must be rejected just like an
    // entry point.
    view fn _bad_view() -> u32 {
        return 2u32;
    }
}
