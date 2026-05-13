import storage_demo.aleo;
program test_storage_demo.aleo;

struct Optional__ATAkdHctJwx:
    is_some as boolean;
    val as boolean;

struct Optional__DmN5CQ9hzeK:
    is_some as boolean;
    val as u64;

struct Optional__3aph2JMPtnA:
    is_some as boolean;
    val as u8;

struct Optional__GfbJUku6XoG:
    is_some as boolean;
    val as i64;

struct Optional__7o6su2Uhzht:
    is_some as boolean;
    val as field;

struct Optional__BnktEVOqVZ2:
    is_some as boolean;
    val as [u32; 4u32];

struct Optional__6wU7KHJhSvv:
    is_some as boolean;
    val as address;

struct Optional__9SsWRr2WG5s:
    is_some as boolean;
    val as group;

struct Optional__7G49nMxvEkY:
    is_some as boolean;
    val as storage_demo.aleo/Point;

struct Optional__JzunLORyB8U:
    is_some as boolean;
    val as u32;

function unset_singletons_default:
    async unset_singletons_default into r0;
    output r0 as test_storage_demo.aleo/unset_singletons_default.future;

finalize unset_singletons_default:
    contains storage_demo.aleo/flag__[false] into r0;
    get.or_use storage_demo.aleo/flag__[false] false into r1;
    ternary r0 r1 false into r2;
    cast r0 r2 into r3 as Optional__ATAkdHctJwx;
    ternary r3.is_some r3.val false into r4;
    is.eq r4 false into r5;
    assert.eq r5 true;
    contains storage_demo.aleo/counter__[false] into r6;
    get.or_use storage_demo.aleo/counter__[false] 0u64 into r7;
    ternary r6 r7 0u64 into r8;
    cast r6 r8 into r9 as Optional__DmN5CQ9hzeK;
    ternary r9.is_some r9.val 0u64 into r10;
    is.eq r10 0u64 into r11;
    assert.eq r11 true;
    contains storage_demo.aleo/tiny__[false] into r12;
    get.or_use storage_demo.aleo/tiny__[false] 0u8 into r13;
    ternary r12 r13 0u8 into r14;
    cast r12 r14 into r15 as Optional__3aph2JMPtnA;
    ternary r15.is_some r15.val 0u8 into r16;
    is.eq r16 0u8 into r17;
    assert.eq r17 true;
    contains storage_demo.aleo/signed__[false] into r18;
    get.or_use storage_demo.aleo/signed__[false] 0i64 into r19;
    ternary r18 r19 0i64 into r20;
    cast r18 r20 into r21 as Optional__GfbJUku6XoG;
    ternary r21.is_some r21.val 0i64 into r22;
    is.eq r22 0i64 into r23;
    assert.eq r23 true;
    contains storage_demo.aleo/marker__[false] into r24;
    get.or_use storage_demo.aleo/marker__[false] 0field into r25;
    ternary r24 r25 0field into r26;
    cast r24 r26 into r27 as Optional__7o6su2Uhzht;
    ternary r27.is_some r27.val 0field into r28;
    is.eq r28 0field into r29;
    assert.eq r29 true;
    contains storage_demo.aleo/quad__[false] into r30;
    cast 0u32 0u32 0u32 0u32 into r31 as [u32; 4u32];
    get.or_use storage_demo.aleo/quad__[false] r31 into r32;
    ternary r30 r32[0u32] r31[0u32] into r33;
    ternary r30 r32[1u32] r31[1u32] into r34;
    ternary r30 r32[2u32] r31[2u32] into r35;
    ternary r30 r32[3u32] r31[3u32] into r36;
    cast r33 r34 r35 r36 into r37 as [u32; 4u32];
    cast r30 r37 into r38 as Optional__BnktEVOqVZ2;
    cast 0u32 0u32 0u32 0u32 into r39 as [u32; 4u32];
    ternary r38.is_some r38.val[0u32] 0u32 into r40;
    ternary r38.is_some r38.val[1u32] 0u32 into r41;
    ternary r38.is_some r38.val[2u32] 0u32 into r42;
    ternary r38.is_some r38.val[3u32] 0u32 into r43;
    cast r40 r41 r42 r43 into r44 as [u32; 4u32];
    is.eq r44 r39 into r45;
    assert.eq r45 true;

function set_and_read_flag:
    call storage_demo.aleo/set_flag true into r0;
    async set_and_read_flag r0 into r1;
    output r1 as test_storage_demo.aleo/set_and_read_flag.future;

finalize set_and_read_flag:
    input r0 as storage_demo.aleo/set_flag.future;
    await r0;
    contains storage_demo.aleo/flag__[false] into r1;
    get.or_use storage_demo.aleo/flag__[false] false into r2;
    ternary r1 r2 false into r3;
    cast r1 r3 into r4 as Optional__ATAkdHctJwx;
    assert.eq r4.is_some true;
    is.eq r4.val true into r5;
    assert.eq r5 true;

function set_and_read_counter:
    call storage_demo.aleo/set_counter 42u64 into r0;
    async set_and_read_counter r0 into r1;
    output r1 as test_storage_demo.aleo/set_and_read_counter.future;

finalize set_and_read_counter:
    input r0 as storage_demo.aleo/set_counter.future;
    await r0;
    contains storage_demo.aleo/counter__[false] into r1;
    get.or_use storage_demo.aleo/counter__[false] 0u64 into r2;
    ternary r1 r2 0u64 into r3;
    cast r1 r3 into r4 as Optional__DmN5CQ9hzeK;
    assert.eq r4.is_some true;
    is.eq r4.val 42u64 into r5;
    assert.eq r5 true;
    contains storage_demo.aleo/counter__[false] into r6;
    get.or_use storage_demo.aleo/counter__[false] 0u64 into r7;
    ternary r6 r7 0u64 into r8;
    cast r6 r8 into r9 as Optional__DmN5CQ9hzeK;
    ternary r9.is_some r9.val 0u64 into r10;
    is.eq r10 42u64 into r11;
    assert.eq r11 true;

function bump_counter_from_unset:
    call storage_demo.aleo/bump_counter into r0;
    async bump_counter_from_unset r0 into r1;
    output r1 as test_storage_demo.aleo/bump_counter_from_unset.future;

finalize bump_counter_from_unset:
    input r0 as storage_demo.aleo/bump_counter.future;
    await r0;
    contains storage_demo.aleo/counter__[false] into r1;
    get.or_use storage_demo.aleo/counter__[false] 0u64 into r2;
    ternary r1 r2 0u64 into r3;
    cast r1 r3 into r4 as Optional__DmN5CQ9hzeK;
    assert.eq r4.is_some true;
    is.eq r4.val 1u64 into r5;
    assert.eq r5 true;

function bump_counter_twice:
    call storage_demo.aleo/bump_counter into r0;
    call storage_demo.aleo/bump_counter into r1;
    async bump_counter_twice r0 r1 into r2;
    output r2 as test_storage_demo.aleo/bump_counter_twice.future;

finalize bump_counter_twice:
    input r0 as storage_demo.aleo/bump_counter.future;
    input r1 as storage_demo.aleo/bump_counter.future;
    await r0;
    await r1;
    contains storage_demo.aleo/counter__[false] into r2;
    get.or_use storage_demo.aleo/counter__[false] 0u64 into r3;
    ternary r2 r3 0u64 into r4;
    cast r2 r4 into r5 as Optional__DmN5CQ9hzeK;
    assert.eq r5.is_some true;
    is.eq r5.val 2u64 into r6;
    assert.eq r6 true;

function clear_counter_resets_default:
    call storage_demo.aleo/set_counter 99u64 into r0;
    call storage_demo.aleo/clear_counter into r1;
    async clear_counter_resets_default r0 r1 into r2;
    output r2 as test_storage_demo.aleo/clear_counter_resets_default.future;

finalize clear_counter_resets_default:
    input r0 as storage_demo.aleo/set_counter.future;
    input r1 as storage_demo.aleo/clear_counter.future;
    await r0;
    await r1;
    contains storage_demo.aleo/counter__[false] into r2;
    get.or_use storage_demo.aleo/counter__[false] 0u64 into r3;
    ternary r2 r3 0u64 into r4;
    cast r2 r4 into r5 as Optional__DmN5CQ9hzeK;
    ternary r5.is_some r5.val 0u64 into r6;
    is.eq r6 0u64 into r7;
    assert.eq r7 true;

function set_and_read_tiny:
    call storage_demo.aleo/set_tiny 255u8 into r0;
    async set_and_read_tiny r0 into r1;
    output r1 as test_storage_demo.aleo/set_and_read_tiny.future;

finalize set_and_read_tiny:
    input r0 as storage_demo.aleo/set_tiny.future;
    await r0;
    contains storage_demo.aleo/tiny__[false] into r1;
    get.or_use storage_demo.aleo/tiny__[false] 0u8 into r2;
    ternary r1 r2 0u8 into r3;
    cast r1 r3 into r4 as Optional__3aph2JMPtnA;
    assert.eq r4.is_some true;
    is.eq r4.val 255u8 into r5;
    assert.eq r5 true;

function set_and_read_signed:
    call storage_demo.aleo/set_signed -1234i64 into r0;
    async set_and_read_signed r0 into r1;
    output r1 as test_storage_demo.aleo/set_and_read_signed.future;

finalize set_and_read_signed:
    input r0 as storage_demo.aleo/set_signed.future;
    await r0;
    contains storage_demo.aleo/signed__[false] into r1;
    get.or_use storage_demo.aleo/signed__[false] 0i64 into r2;
    ternary r1 r2 0i64 into r3;
    cast r1 r3 into r4 as Optional__GfbJUku6XoG;
    assert.eq r4.is_some true;
    is.eq r4.val -1234i64 into r5;
    assert.eq r5 true;

function set_and_read_tag:
    call storage_demo.aleo/set_tag aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px into r0;
    async set_and_read_tag r0 aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px into r1;
    output r1 as test_storage_demo.aleo/set_and_read_tag.future;

finalize set_and_read_tag:
    input r0 as storage_demo.aleo/set_tag.future;
    input r1 as address.public;
    await r0;
    contains storage_demo.aleo/tag__[false] into r2;
    get.or_use storage_demo.aleo/tag__[false] aleo1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3ljyzc into r3;
    ternary r2 r3 aleo1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3ljyzc into r4;
    cast r2 r4 into r5 as Optional__6wU7KHJhSvv;
    assert.eq r5.is_some true;
    is.eq r5.val r1 into r6;
    assert.eq r6 true;

function set_and_read_marker:
    call storage_demo.aleo/set_marker 7field into r0;
    async set_and_read_marker r0 into r1;
    output r1 as test_storage_demo.aleo/set_and_read_marker.future;

finalize set_and_read_marker:
    input r0 as storage_demo.aleo/set_marker.future;
    await r0;
    contains storage_demo.aleo/marker__[false] into r1;
    get.or_use storage_demo.aleo/marker__[false] 0field into r2;
    ternary r1 r2 0field into r3;
    cast r1 r3 into r4 as Optional__7o6su2Uhzht;
    assert.eq r4.is_some true;
    is.eq r4.val 7field into r5;
    assert.eq r5 true;

function set_and_read_pivot:
    call storage_demo.aleo/set_pivot 2group into r0;
    async set_and_read_pivot r0 2group into r1;
    output r1 as test_storage_demo.aleo/set_and_read_pivot.future;

finalize set_and_read_pivot:
    input r0 as storage_demo.aleo/set_pivot.future;
    input r1 as group.public;
    await r0;
    contains storage_demo.aleo/pivot__[false] into r2;
    get.or_use storage_demo.aleo/pivot__[false] 0group into r3;
    ternary r2 r3 0group into r4;
    cast r2 r4 into r5 as Optional__9SsWRr2WG5s;
    assert.eq r5.is_some true;
    is.eq r5.val r1 into r6;
    assert.eq r6 true;

function set_and_read_point:
    cast -5i64 9i64 into r0 as storage_demo.aleo/Point;
    call storage_demo.aleo/set_point r0 into r1;
    async set_and_read_point r1 into r2;
    output r2 as test_storage_demo.aleo/set_and_read_point.future;

finalize set_and_read_point:
    input r0 as storage_demo.aleo/set_point.future;
    await r0;
    contains storage_demo.aleo/point__[false] into r1;
    cast 0i64 0i64 into r2 as storage_demo.aleo/Point;
    get.or_use storage_demo.aleo/point__[false] r2 into r3;
    cast 0i64 0i64 into r4 as storage_demo.aleo/Point;
    ternary r1 r3.x r4.x into r5;
    ternary r1 r3.y r4.y into r6;
    cast r5 r6 into r7 as storage_demo.aleo/Point;
    cast r1 r7 into r8 as Optional__7G49nMxvEkY;
    assert.eq r8.is_some true;
    is.eq r8.val.x -5i64 into r9;
    assert.eq r9 true;
    is.eq r8.val.y 9i64 into r10;
    assert.eq r10 true;

function set_and_read_quad:
    cast 1u32 2u32 3u32 4u32 into r0 as [u32; 4u32];
    call storage_demo.aleo/set_quad r0 into r1;
    async set_and_read_quad r1 into r2;
    output r2 as test_storage_demo.aleo/set_and_read_quad.future;

finalize set_and_read_quad:
    input r0 as storage_demo.aleo/set_quad.future;
    await r0;
    contains storage_demo.aleo/quad__[false] into r1;
    cast 0u32 0u32 0u32 0u32 into r2 as [u32; 4u32];
    get.or_use storage_demo.aleo/quad__[false] r2 into r3;
    ternary r1 r3[0u32] r2[0u32] into r4;
    ternary r1 r3[1u32] r2[1u32] into r5;
    ternary r1 r3[2u32] r2[2u32] into r6;
    ternary r1 r3[3u32] r2[3u32] into r7;
    cast r4 r5 r6 r7 into r8 as [u32; 4u32];
    cast r1 r8 into r9 as Optional__BnktEVOqVZ2;
    assert.eq r9.is_some true;
    is.eq r9.val[0u32] 1u32 into r10;
    assert.eq r10 true;
    is.eq r9.val[1u32] 2u32 into r11;
    assert.eq r11 true;
    is.eq r9.val[2u32] 3u32 into r12;
    assert.eq r12 true;
    is.eq r9.val[3u32] 4u32 into r13;
    assert.eq r13 true;

function vector_starts_empty:
    async vector_starts_empty into r0;
    output r0 as test_storage_demo.aleo/vector_starts_empty.future;

finalize vector_starts_empty:
    get.or_use storage_demo.aleo/history__len__[false] 0u32 into r0;
    is.eq r0 0u32 into r1;
    assert.eq r1 true;
    get.or_use storage_demo.aleo/history__len__[false] 0u32 into r2;
    lt 0u32 r2 into r3;
    get.or_use storage_demo.aleo/history__[0u32] 0u32 into r4;
    ternary r3 r4 0u32 into r5;
    cast r3 r5 into r6 as Optional__JzunLORyB8U;
    ternary r6.is_some r6.val 0u32 into r7;
    is.eq r7 0u32 into r8;
    assert.eq r8 true;

function push_then_len_and_get:
    call storage_demo.aleo/push_history 10u32 into r0;
    call storage_demo.aleo/push_history 20u32 into r1;
    call storage_demo.aleo/push_history 30u32 into r2;
    async push_then_len_and_get r0 r1 r2 into r3;
    output r3 as test_storage_demo.aleo/push_then_len_and_get.future;

finalize push_then_len_and_get:
    input r0 as storage_demo.aleo/push_history.future;
    input r1 as storage_demo.aleo/push_history.future;
    input r2 as storage_demo.aleo/push_history.future;
    await r0;
    await r1;
    await r2;
    get.or_use storage_demo.aleo/history__len__[false] 0u32 into r3;
    is.eq r3 3u32 into r4;
    assert.eq r4 true;
    get.or_use storage_demo.aleo/history__len__[false] 0u32 into r5;
    lt 0u32 r5 into r6;
    get.or_use storage_demo.aleo/history__[0u32] 0u32 into r7;
    ternary r6 r7 0u32 into r8;
    cast r6 r8 into r9 as Optional__JzunLORyB8U;
    assert.eq r9.is_some true;
    is.eq r9.val 10u32 into r10;
    assert.eq r10 true;
    get.or_use storage_demo.aleo/history__len__[false] 0u32 into r11;
    lt 1u32 r11 into r12;
    get.or_use storage_demo.aleo/history__[1u32] 0u32 into r13;
    ternary r12 r13 0u32 into r14;
    cast r12 r14 into r15 as Optional__JzunLORyB8U;
    assert.eq r15.is_some true;
    is.eq r15.val 20u32 into r16;
    assert.eq r16 true;
    get.or_use storage_demo.aleo/history__len__[false] 0u32 into r17;
    lt 2u32 r17 into r18;
    get.or_use storage_demo.aleo/history__[2u32] 0u32 into r19;
    ternary r18 r19 0u32 into r20;
    cast r18 r20 into r21 as Optional__JzunLORyB8U;
    assert.eq r21.is_some true;
    is.eq r21.val 30u32 into r22;
    assert.eq r22 true;
    get.or_use storage_demo.aleo/history__len__[false] 0u32 into r23;
    lt 3u32 r23 into r24;
    get.or_use storage_demo.aleo/history__[3u32] 0u32 into r25;
    ternary r24 r25 0u32 into r26;
    cast r24 r26 into r27 as Optional__JzunLORyB8U;
    ternary r27.is_some r27.val 0u32 into r28;
    is.eq r28 0u32 into r29;
    assert.eq r29 true;

function pop_drops_last_element:
    call storage_demo.aleo/push_history 7u32 into r0;
    call storage_demo.aleo/push_history 8u32 into r1;
    call storage_demo.aleo/pop_history into r2;
    async pop_drops_last_element r0 r1 r2 into r3;
    output r3 as test_storage_demo.aleo/pop_drops_last_element.future;

finalize pop_drops_last_element:
    input r0 as storage_demo.aleo/push_history.future;
    input r1 as storage_demo.aleo/push_history.future;
    input r2 as storage_demo.aleo/pop_history.future;
    await r0;
    await r1;
    await r2;
    get.or_use storage_demo.aleo/history__len__[false] 0u32 into r3;
    is.eq r3 1u32 into r4;
    assert.eq r4 true;
    get.or_use storage_demo.aleo/history__len__[false] 0u32 into r5;
    lt 0u32 r5 into r6;
    get.or_use storage_demo.aleo/history__[0u32] 0u32 into r7;
    ternary r6 r7 0u32 into r8;
    cast r6 r8 into r9 as Optional__JzunLORyB8U;
    assert.eq r9.is_some true;
    is.eq r9.val 7u32 into r10;
    assert.eq r10 true;

function set_replaces_slot:
    call storage_demo.aleo/push_history 100u32 into r0;
    call storage_demo.aleo/push_history 200u32 into r1;
    call storage_demo.aleo/set_history 1u32 999u32 into r2;
    async set_replaces_slot r0 r1 r2 into r3;
    output r3 as test_storage_demo.aleo/set_replaces_slot.future;

finalize set_replaces_slot:
    input r0 as storage_demo.aleo/push_history.future;
    input r1 as storage_demo.aleo/push_history.future;
    input r2 as storage_demo.aleo/set_history.future;
    await r0;
    await r1;
    await r2;
    get.or_use storage_demo.aleo/history__len__[false] 0u32 into r3;
    is.eq r3 2u32 into r4;
    assert.eq r4 true;
    get.or_use storage_demo.aleo/history__len__[false] 0u32 into r5;
    lt 0u32 r5 into r6;
    get.or_use storage_demo.aleo/history__[0u32] 0u32 into r7;
    ternary r6 r7 0u32 into r8;
    cast r6 r8 into r9 as Optional__JzunLORyB8U;
    assert.eq r9.is_some true;
    is.eq r9.val 100u32 into r10;
    assert.eq r10 true;
    get.or_use storage_demo.aleo/history__len__[false] 0u32 into r11;
    lt 1u32 r11 into r12;
    get.or_use storage_demo.aleo/history__[1u32] 0u32 into r13;
    ternary r12 r13 0u32 into r14;
    cast r12 r14 into r15 as Optional__JzunLORyB8U;
    assert.eq r15.is_some true;
    is.eq r15.val 999u32 into r16;
    assert.eq r16 true;

function swap_remove_pulls_last:
    call storage_demo.aleo/push_history 1u32 into r0;
    call storage_demo.aleo/push_history 2u32 into r1;
    call storage_demo.aleo/push_history 3u32 into r2;
    call storage_demo.aleo/push_history 4u32 into r3;
    call storage_demo.aleo/swap_remove_history 1u32 into r4;
    async swap_remove_pulls_last r0 r1 r2 r3 r4 into r5;
    output r5 as test_storage_demo.aleo/swap_remove_pulls_last.future;

finalize swap_remove_pulls_last:
    input r0 as storage_demo.aleo/push_history.future;
    input r1 as storage_demo.aleo/push_history.future;
    input r2 as storage_demo.aleo/push_history.future;
    input r3 as storage_demo.aleo/push_history.future;
    input r4 as storage_demo.aleo/swap_remove_history.future;
    await r0;
    await r1;
    await r2;
    await r3;
    await r4;
    get.or_use storage_demo.aleo/history__len__[false] 0u32 into r5;
    is.eq r5 3u32 into r6;
    assert.eq r6 true;
    get.or_use storage_demo.aleo/history__len__[false] 0u32 into r7;
    lt 0u32 r7 into r8;
    get.or_use storage_demo.aleo/history__[0u32] 0u32 into r9;
    ternary r8 r9 0u32 into r10;
    cast r8 r10 into r11 as Optional__JzunLORyB8U;
    assert.eq r11.is_some true;
    is.eq r11.val 1u32 into r12;
    assert.eq r12 true;
    get.or_use storage_demo.aleo/history__len__[false] 0u32 into r13;
    lt 1u32 r13 into r14;
    get.or_use storage_demo.aleo/history__[1u32] 0u32 into r15;
    ternary r14 r15 0u32 into r16;
    cast r14 r16 into r17 as Optional__JzunLORyB8U;
    assert.eq r17.is_some true;
    is.eq r17.val 4u32 into r18;
    assert.eq r18 true;
    get.or_use storage_demo.aleo/history__len__[false] 0u32 into r19;
    lt 2u32 r19 into r20;
    get.or_use storage_demo.aleo/history__[2u32] 0u32 into r21;
    ternary r20 r21 0u32 into r22;
    cast r20 r22 into r23 as Optional__JzunLORyB8U;
    assert.eq r23.is_some true;
    is.eq r23.val 3u32 into r24;
    assert.eq r24 true;

function clear_resets_length:
    call storage_demo.aleo/push_history 1u32 into r0;
    call storage_demo.aleo/push_history 2u32 into r1;
    call storage_demo.aleo/clear_history into r2;
    async clear_resets_length r0 r1 r2 into r3;
    output r3 as test_storage_demo.aleo/clear_resets_length.future;

finalize clear_resets_length:
    input r0 as storage_demo.aleo/push_history.future;
    input r1 as storage_demo.aleo/push_history.future;
    input r2 as storage_demo.aleo/clear_history.future;
    await r0;
    await r1;
    await r2;
    get.or_use storage_demo.aleo/history__len__[false] 0u32 into r3;
    is.eq r3 0u32 into r4;
    assert.eq r4 true;
    get.or_use storage_demo.aleo/history__len__[false] 0u32 into r5;
    lt 0u32 r5 into r6;
    get.or_use storage_demo.aleo/history__[0u32] 0u32 into r7;
    ternary r6 r7 0u32 into r8;
    cast r6 r8 into r9 as Optional__JzunLORyB8U;
    ternary r9.is_some r9.val 0u32 into r10;
    is.eq r10 0u32 into r11;
    assert.eq r11 true;

function unwrap_unset_singleton_halts:
    async unwrap_unset_singleton_halts into r0;
    output r0 as test_storage_demo.aleo/unwrap_unset_singleton_halts.future;

finalize unwrap_unset_singleton_halts:
    contains storage_demo.aleo/counter__[false] into r0;
    get.or_use storage_demo.aleo/counter__[false] 0u64 into r1;
    ternary r0 r1 0u64 into r2;
    cast r0 r2 into r3 as Optional__DmN5CQ9hzeK;
    assert.eq r3.is_some true;
    is.eq r3.val 0u64 into r4;
    assert.eq r4 true;

function unwrap_oob_get_halts:
    async unwrap_oob_get_halts into r0;
    output r0 as test_storage_demo.aleo/unwrap_oob_get_halts.future;

finalize unwrap_oob_get_halts:
    get.or_use storage_demo.aleo/history__len__[false] 0u32 into r0;
    lt 0u32 r0 into r1;
    get.or_use storage_demo.aleo/history__[0u32] 0u32 into r2;
    ternary r1 r2 0u32 into r3;
    cast r1 r3 into r4 as Optional__JzunLORyB8U;
    assert.eq r4.is_some true;
    is.eq r4.val 0u32 into r5;
    assert.eq r5 true;

constructor:
    assert.eq edition 0u16;
