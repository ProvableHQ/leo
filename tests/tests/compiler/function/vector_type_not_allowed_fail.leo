
// Vector as function parameter
fn foo(a: [u32]) -> u32 {
    return 0u32;
}

// Vector as function return type
fn bar() -> [u32] {
    return 0u32;
}

// Vector as struct field
struct MyStruct {
    data: [u32]
}

// Vector nested in array
fn baz(a: [[u32]; 3]) -> u32 {
    return 0u32;
}

program test.aleo {
    // Vector as record field
    record MyRecord {
        owner: address,
        data: [u32]
    }

    // Vector as mapping key type
    mapping my_map: [u32] => u32;

    // Vector as mapping value type
    mapping my_map2: u32 => [u32];

    fn main() -> Final {
        return final {};
    }
}
