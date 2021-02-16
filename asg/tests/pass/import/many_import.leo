import test-import.( // local import
    Point,
    foo,
);

import bar.( // imports directory import
    Bar,
    baz.(Baz, Bazzar),
    bat.bat.Bat,
);

import car.Car; // imports directory import

function main() {
    let point = Point { x: 1u32, y: 1u32 };
    let foo = foo();

    let bar = Bar { r: 1u32 };
    let baz = Baz { z: 1u32 };
    let bazzar = Bazzar { a: 1u32 };
    let bat = Bat { t: 1u32 };

    let car = Car { c: 1u32 };

    console.assert(car.c == 1u32);
}