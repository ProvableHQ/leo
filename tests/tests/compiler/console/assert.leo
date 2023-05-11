/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    struct Foo {
        a: u8;
    }
    
    record Token {
        // The token owner.
        owner: address,
        // The token amount.
        amount: u64,
    }
    
    
    transition main(a: bool, foo: Foo, token: Token) -> bool {
        assert_eq(a, true);
        assert_neq(a, false);
        assert(a);
    
        assert_eq(foo, Foo { a: 0u8 });
    
        assert_neq(token, Token {
            owner: aleo1lfapwg53y5enqpt0d8cnef4g8lj7l6g9uhkkma23qyv6jm4ppyfq50regr,
            amount: 0u64,
        });
        return a == true;
    }
}
