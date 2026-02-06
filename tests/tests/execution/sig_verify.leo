/*
seed = 123456789
min_height = 16

[case]
program = "test.aleo"
function = "main"
input = [ "aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px", "5field" ]

[case]
program = "test.aleo"
function = "verify_field"
input = [ "sign1vck9vl9w0w9xh3z9dph5qmyqacqapyuve7jfqvgfyslglx92auqmr6965sdn7wp4n9nahysxps633j9d2g4rgs2zhxlz75sxm8af7qvr22qjwn4zc0pzv87twjygsz9m7ekljmuw4jpzf68rwuq99r0tp735vs6220q7tp60nr7llkwstcvu49wdhydx5x2s3sftjskzawhqvqcj6tl", "aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px", "5field"]

[case]
program = "test.aleo"
function = "verify_foo"
input = [ "sign13ppv2557dkltpjnks7sxqu7lm0wnnlz8vw4m5we2c54synkj75qe2c6a7u4kpz5myc8dr77a205lvrkwymuyjvg6z3k5xmwfel8v2qur22qjwn4zc0pzv87twjygsz9m7ekljmuw4jpzf68rwuq99r0tp735vs6220q7tp60nr7llkwstcvu49wdhydx5x2s3sftjskzawhqvcudqf8", "aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px", "{ a: 5u8, b: 0scalar }" ]

[case]
program = "test.aleo"
function = "literal_ops_on_signature"
input = [ "true", "sign1vck9vl9w0w9xh3z9dph5qmyqacqapyuve7jfqvgfyslglx92auqmr6965sdn7wp4n9nahysxps633j9d2g4rgs2zhxlz75sxm8af7qvr22qjwn4zc0pzv87twjygsz9m7ekljmuw4jpzf68rwuq99r0tp735vs6220q7tp60nr7llkwstcvu49wdhydx5x2s3sftjskzawhqvqcj6tl", "sign1fmqr4tj252phr3az3c0cwe0w7n28ctxm3c03dq9j0em9s6zfksqp576zqqx3676qtv8nt5yyy2ad9wnxgw5kh6tnmu6x4gf55w3mwqvr22qjwn4zc0pzv87twjygsz9m7ekljmuw4jpzf68rwuq99r0tp735vs6220q7tp60nr7llkwstcvu49wdhydx5x2s3sftjskzawhqvmjn8nz" ]

[case]
program = "test.aleo"
function = "literal_test"
input = [ "sign1vck9vl9w0w9xh3z9dph5qmyqacqapyuve7jfqvgfyslglx92auqmr6965sdn7wp4n9nahysxps633j9d2g4rgs2zhxlz75sxm8af7qvr22qjwn4zc0pzv87twjygsz9m7ekljmuw4jpzf68rwuq99r0tp735vs6220q7tp60nr7llkwstcvu49wdhydx5x2s3sftjskzawhqvqcj6tl" ]
*/

program test.aleo {

    struct foo {
        a: u8,
        b: scalar
    }

    const SIG_5: signature =
      sign1vck9vl9w0w9xh3z9dph5qmyqacqapyuve7jfqvgfyslglx92auqmr6965sdn7wp4n9nahysxps633j9d2g4rgs2zhxlz75sxm8af7qvr22qjwn4zc0pzv87twjygsz9m7ekljmuw4jpzf68rwuq99r0tp735vs6220q7tp60nr7llkwstcvu49wdhydx5x2s3sftjskzawhqvqcj6tl;

    // literal/const method call
    transition main(a: address, v: field) -> bool {
        return SIG_5.verify(a, v);
    }

    transition verify_field(s: signature, a: address, v: field) {
        let first = signature::verify(s, a, v);
        let second = s.verify(a, v);
        assert_eq(first, second);
        assert(first);
    }

    transition verify_foo(s: signature, a: address, v: foo) {
        let first = signature::verify(s, a, v);
        let second = s.verify(a, v);
        assert_eq(first, second);
        assert(first);
    }

    transition literal_ops_on_signature(a: bool, b: signature, c: signature) -> signature {
        assert(b == b);
        assert(c == c);
        assert(b != c);
        return a ? b : c;
    }

    transition literal_test(s0: signature) -> (bool, bool, signature) {
        let input_eq_const = s0 == SIG_5;
        assert(input_eq_const);

        let ternary = true ? s0 : SIG_5;

        return (input_eq_const, s0 == ternary, ternary);
    }
}
