
// Test 1 
const X: u32 = 5;

// Test 2 
struct Foo {
    x: u32, y: u32,
}

// Test 3 
@inline
fn foo() {}

// Test 4 
const X: u32 = 5;
struct Foo {
    x: u32, y: u32,
}
@inline
fn foo() {}
struct Bar {
    x: u32, y: u32,
}
const Y: u32 = 5;
@inline
fn bar() {}
const Z: u32 = 5;
