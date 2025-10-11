program child1.aleo {
    struct Foo {
        x: u32,
        y: u32,
        z: u32,
    }

    transition main1(foo: Foo) { }

    transition main2(x: u32, y: u32, z: u32) -> Foo {
        return Foo { x, y, z };
    }
}

// --- Next Program --- //

program child2.aleo {
    struct Foo {
        x: u32,
        y: u32,
        z: u32,
        w: u32,
    }

    transition main1(foo: Foo) { }

    transition main2(x: u32, y: u32, z: u32, w: u32) -> Foo {
        return Foo { x, y, z, w };
    }
}

// --- Next Program --- //

import child1.aleo;
import child2.aleo;

program external_structs.aleo {
    struct Foo {
        x: u32,
        y: u32,
    }

    transition main1(local: Foo, 
                    external1: child1.aleo/Foo,
                    external2: child2.aleo/Foo) { }

    transition main2() -> u32 {
        let external1 = child1.aleo/main2(1, 2, 3);
        let external2 = child2.aleo/main2(1, 2, 3, 4);
        return external1.x + external1.y + external1.z
             + external2.x + external2.y + external2.z + external2.w;
    }
}
