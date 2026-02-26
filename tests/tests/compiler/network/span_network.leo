// The purpose of this test is to make sure reporting the `no_transition` error
// works - the `network` member of `ProgramScope` was not getting the correct
// `Span` during parsing, and consequently when display an error with multiple
// source files, a panic could happen.

program child.aleo {
    fn main() {}
}

// --- Next Program --- //

import child.aleo;

program parent.aleo {}

fn f(x :u8) {
    assert_eq(x, x);
}

