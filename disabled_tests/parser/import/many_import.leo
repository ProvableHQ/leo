/*
namespace: ParseImport
expectation: Pass
*/

import tes_import.(
    Point,
    foo,
);

import bar.(
    Bar,
    baz.(Baz, Bazzar),
    bat.bat.Bat,
);
