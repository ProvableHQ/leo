/*
seed = 123456789
[case]
program = "external_structs.aleo"
function = "main1"
input = ["{x:11u32, y:22u32}", "{x:33u32, y:44u32, z:55u32}", "{x:66u32, y:77u32, z:88u32, w:99u32}"]
[case]
program = "external_structs.aleo"
function = "main2"
input = []
*/

struct Foo {
    x: u32,
    y: u32,
    z: u32,
}

program child1.aleo {
    fn main1(foo: Foo) { }

    fn main2(x: u32, y: u32, z: u32) -> Foo {
        return Foo { x, y, z };
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //

struct Foo {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

program child2.aleo {
    fn main1(foo: Foo) { }

    fn main2(x: u32, y: u32, z: u32, w: u32) -> Foo {
        return Foo { x, y, z, w };
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //

import child1.aleo;
import child2.aleo;

struct Foo {
    x: u32,
    y: u32,
}
program external_structs.aleo {
    fn main1(local: Foo, 
                    external1: child1.aleo/Foo,
                    external2: child2.aleo/Foo) { 
        assert_eq(
               local.x + local.y
             + external1.x + external1.y + external1.z
             + external2.x + external2.y + external2.z + external2.w, 
             11u32 + 22 + 33 + 44 + 55 + 66 + 77 + 88 + 99
        );
    }

    fn main2() -> public u32 {
        let external1 = child1.aleo/main2(1, 2, 3);
        let external2 = child2.aleo/main2(1, 2, 3, 4);
        return external1.x + external1.y + external1.z
             + external2.x + external2.y + external2.z + external2.w;
    }

    @noupgrade
    constructor() {}
}
