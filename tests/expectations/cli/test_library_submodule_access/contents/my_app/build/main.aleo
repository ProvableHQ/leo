program my_app.aleo;

struct Point__AyDZp1FeJ4R:
    x as u32;
    y as u32;

struct Rect__FrGrwpvu3zJ:
    top_left as Point__AyDZp1FeJ4R;
    bottom_right as Point__AyDZp1FeJ4R;

function rect_area:
    input r0 as Rect__FrGrwpvu3zJ.private;
    sub r0.bottom_right.x r0.top_left.x into r1;
    sub r0.bottom_right.y r0.top_left.y into r2;
    mul r1 r2 into r3;
    output r3 as u32.private;

function make_rect:
    input r0 as u32.private;
    input r1 as u32.private;
    input r2 as u32.private;
    input r3 as u32.private;
    cast r0 r1 into r4 as Point__AyDZp1FeJ4R;
    cast r2 r3 into r5 as Point__AyDZp1FeJ4R;
    cast r4 r5 into r6 as Rect__FrGrwpvu3zJ;
    output r6 as Rect__FrGrwpvu3zJ.private;

function unit_side:
    output 1u32 as u32.private;

function scaled_area:
    input r0 as Rect__FrGrwpvu3zJ.private;
    input r1 as u32.private;
    sub r0.bottom_right.x r0.top_left.x into r2;
    sub r0.bottom_right.y r0.top_left.y into r3;
    mul r2 r3 into r4;
    mul r4 r1 into r5;
    output r5 as u32.private;

constructor:
    assert.eq edition 0u16;
