/*
namespace: Parse
expectation: Pass
*/

import test-import.( // local import
    Point,
    foo,
);

import bar.( // imports directory import
    Bar,
    baz.(Baz, Bazzar),
    bat.bat.Bat,
);
