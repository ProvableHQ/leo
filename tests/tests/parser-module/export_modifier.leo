// Test: export fn at module level
export fn foo() {}

// Test: export struct
export struct Foo {
    x: u32, y: u32,
}

// Test: export const
export const X: u32 = 5;

// Test: export final fn at module level
export final fn finalize_only() {}

// Test: export on multiple items in one module
export const A: u32 = 1;
export const B: u32 = 2;
export struct Pair { x: u32, y: u32 }
export fn add(a: u32, b: u32) -> u32 { return a + b; }

// Test: mix of export and non-export items in the same module
const PRIVATE_CONST: u32 = 1;
export const PUB_CONST: u32 = 2;
fn private_helper() -> u32 { return 0u32; }
export fn public_api() -> u32 { return private_helper() + PUB_CONST; }
