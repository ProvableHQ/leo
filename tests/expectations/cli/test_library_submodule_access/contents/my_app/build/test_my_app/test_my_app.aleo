import my_app.aleo;
program test_my_app.aleo;

struct Point__AyDZp1FeJ4R:
    x as u32;
    y as u32;

struct Rect__FrGrwpvu3zJ:
    top_left as Point__AyDZp1FeJ4R;
    bottom_right as Point__AyDZp1FeJ4R;

function test_rect_area:
    call my_app.aleo/make_rect 0u32 0u32 3u32 4u32 into r0;
    call my_app.aleo/rect_area r0 into r1;
    assert.eq r1 12u32;

function test_unit_side:
    call my_app.aleo/unit_side into r0;
    assert.eq r0 1u32;

function test_scaled_area:
    call my_app.aleo/make_rect 0u32 0u32 2u32 3u32 into r0;
    call my_app.aleo/scaled_area r0 5u32 into r1;
    assert.eq r1 30u32;

constructor:
    assert.eq edition 0u16;
