/*
namespace: Compile
expectation: Pass
*/

circuit TestMe {
    x: u8,

    function test_me(mut self) -> u8 {
        self.x += 1;
        return self.x;
    }

    function new() -> Self {
        return Self { x: 1u8 };
    }
}

function my_fn() -> TestMe {
  return TestMe { x: 0u8 };
}

function main () {
    const t = TestMe {x: 6u8}.test_me();
    console.assert(t == 7u8);
    const u = my_fn().test_me();
    console.assert(u == 1u8);
    const v = TestMe::new().test_me();
    console.assert(v == 2u8);
}
