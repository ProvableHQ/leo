import test_import.*; // local import

import bar.*;         // imports directory import
import bar.baz.*;     // imports directory import
import bar.bat.bat.*; // imports directory import
import car.*;         // imports directory import

function main() -> u32 {
    const point = Point { x: 1u32, y: 1u32 };
    const foo = foo();

    const bar = Bar { r: 1u32 };
    const bat = Bat { t: 1u32 };
    const baz = Baz { z: 1u32 };

    const car = Car { c: 1u32 };

    return car.c
}