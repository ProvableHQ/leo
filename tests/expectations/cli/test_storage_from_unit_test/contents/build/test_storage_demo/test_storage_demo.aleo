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
    cast true r1 into r2 as Optional__ATAkdHctJwx;
    ternary r0 r2.is_some false into r3;
    ternary r0 r1 false into r4;
    cast r3 r4 into r5 as Optional__ATAkdHctJwx;
    ternary r5.is_some r5.val false into r6;
    is.eq r6 false into r7;
    assert.eq r7 true;
    contains storage_demo.aleo/counter__[false] into r8;
    get.or_use storage_demo.aleo/counter__[false] 0u64 into r9;
    cast true r9 into r10 as Optional__DmN5CQ9hzeK;
    ternary r8 r10.is_some false into r11;
    ternary r8 r9 0u64 into r12;
    cast r11 r12 into r13 as Optional__DmN5CQ9hzeK;
    ternary r13.is_some r13.val 0u64 into r14;
    is.eq r14 0u64 into r15;
    assert.eq r15 true;
    contains storage_demo.aleo/tiny__[false] into r16;
    get.or_use storage_demo.aleo/tiny__[false] 0u8 into r17;
    cast true r17 into r18 as Optional__3aph2JMPtnA;
    ternary r16 r18.is_some false into r19;
    ternary r16 r17 0u8 into r20;
    cast r19 r20 into r21 as Optional__3aph2JMPtnA;
    ternary r21.is_some r21.val 0u8 into r22;
    is.eq r22 0u8 into r23;
    assert.eq r23 true;
    contains storage_demo.aleo/signed__[false] into r24;
    get.or_use storage_demo.aleo/signed__[false] 0i64 into r25;
    cast true r25 into r26 as Optional__GfbJUku6XoG;
    ternary r24 r26.is_some false into r27;
    ternary r24 r25 0i64 into r28;
    cast r27 r28 into r29 as Optional__GfbJUku6XoG;
    ternary r29.is_some r29.val 0i64 into r30;
    is.eq r30 0i64 into r31;
    assert.eq r31 true;
    contains storage_demo.aleo/marker__[false] into r32;
    get.or_use storage_demo.aleo/marker__[false] 0field into r33;
    cast true r33 into r34 as Optional__7o6su2Uhzht;
    ternary r32 r34.is_some false into r35;
    ternary r32 r33 0field into r36;
    cast r35 r36 into r37 as Optional__7o6su2Uhzht;
    ternary r37.is_some r37.val 0field into r38;
    is.eq r38 0field into r39;
    assert.eq r39 true;
    contains storage_demo.aleo/quad__[false] into r40;
    cast 0u32 0u32 0u32 0u32 into r41 as [u32; 4u32];
    get.or_use storage_demo.aleo/quad__[false] r41 into r42;
    cast true r42 into r43 as Optional__BnktEVOqVZ2;
    ternary r40 r42[0u32] r41[0u32] into r44;
    ternary r40 r42[1u32] r41[1u32] into r45;
    ternary r40 r42[2u32] r41[2u32] into r46;
    ternary r40 r42[3u32] r41[3u32] into r47;
    cast r44 r45 r46 r47 into r48 as [u32; 4u32];
    ternary r40 r43.is_some false into r49;
    cast r49 r48 into r50 as Optional__BnktEVOqVZ2;
    cast 0u32 0u32 0u32 0u32 into r51 as [u32; 4u32];
    ternary r50.is_some r50.val[0u32] 0u32 into r52;
    ternary r50.is_some r50.val[1u32] 0u32 into r53;
    ternary r50.is_some r50.val[2u32] 0u32 into r54;
    ternary r50.is_some r50.val[3u32] 0u32 into r55;
    cast r52 r53 r54 r55 into r56 as [u32; 4u32];
    is.eq r56 r51 into r57;
    assert.eq r57 true;

function set_and_read_flag:
    call storage_demo.aleo/set_flag true into r0;
    async set_and_read_flag r0 into r1;
    output r1 as test_storage_demo.aleo/set_and_read_flag.future;

finalize set_and_read_flag:
    input r0 as storage_demo.aleo/set_flag.future;
    await r0;
    contains storage_demo.aleo/flag__[false] into r1;
    get.or_use storage_demo.aleo/flag__[false] false into r2;
    cast true r2 into r3 as Optional__ATAkdHctJwx;
    ternary r1 r3.is_some false into r4;
    ternary r1 r2 false into r5;
    cast r4 r5 into r6 as Optional__ATAkdHctJwx;
    assert.eq r6.is_some true;
    is.eq r6.val true into r7;
    assert.eq r7 true;

function set_and_read_counter:
    call storage_demo.aleo/set_counter 42u64 into r0;
    async set_and_read_counter r0 into r1;
    output r1 as test_storage_demo.aleo/set_and_read_counter.future;

finalize set_and_read_counter:
    input r0 as storage_demo.aleo/set_counter.future;
    await r0;
    contains storage_demo.aleo/counter__[false] into r1;
    get.or_use storage_demo.aleo/counter__[false] 0u64 into r2;
    cast true r2 into r3 as Optional__DmN5CQ9hzeK;
    ternary r1 r3.is_some false into r4;
    ternary r1 r2 0u64 into r5;
    cast r4 r5 into r6 as Optional__DmN5CQ9hzeK;
    assert.eq r6.is_some true;
    is.eq r6.val 42u64 into r7;
    assert.eq r7 true;
    contains storage_demo.aleo/counter__[false] into r8;
    get.or_use storage_demo.aleo/counter__[false] 0u64 into r9;
    cast true r9 into r10 as Optional__DmN5CQ9hzeK;
    ternary r8 r10.is_some false into r11;
    ternary r8 r9 0u64 into r12;
    cast r11 r12 into r13 as Optional__DmN5CQ9hzeK;
    ternary r13.is_some r13.val 0u64 into r14;
    is.eq r14 42u64 into r15;
    assert.eq r15 true;

function bump_counter_from_unset:
    call storage_demo.aleo/bump_counter into r0;
    async bump_counter_from_unset r0 into r1;
    output r1 as test_storage_demo.aleo/bump_counter_from_unset.future;

finalize bump_counter_from_unset:
    input r0 as storage_demo.aleo/bump_counter.future;
    await r0;
    contains storage_demo.aleo/counter__[false] into r1;
    get.or_use storage_demo.aleo/counter__[false] 0u64 into r2;
    cast true r2 into r3 as Optional__DmN5CQ9hzeK;
    ternary r1 r3.is_some false into r4;
    ternary r1 r2 0u64 into r5;
    cast r4 r5 into r6 as Optional__DmN5CQ9hzeK;
    assert.eq r6.is_some true;
    is.eq r6.val 1u64 into r7;
    assert.eq r7 true;

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
    cast true r3 into r4 as Optional__DmN5CQ9hzeK;
    ternary r2 r4.is_some false into r5;
    ternary r2 r3 0u64 into r6;
    cast r5 r6 into r7 as Optional__DmN5CQ9hzeK;
    assert.eq r7.is_some true;
    is.eq r7.val 2u64 into r8;
    assert.eq r8 true;

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
    cast true r3 into r4 as Optional__DmN5CQ9hzeK;
    ternary r2 r4.is_some false into r5;
    ternary r2 r3 0u64 into r6;
    cast r5 r6 into r7 as Optional__DmN5CQ9hzeK;
    ternary r7.is_some r7.val 0u64 into r8;
    is.eq r8 0u64 into r9;
    assert.eq r9 true;

function set_and_read_tiny:
    call storage_demo.aleo/set_tiny 255u8 into r0;
    async set_and_read_tiny r0 into r1;
    output r1 as test_storage_demo.aleo/set_and_read_tiny.future;

finalize set_and_read_tiny:
    input r0 as storage_demo.aleo/set_tiny.future;
    await r0;
    contains storage_demo.aleo/tiny__[false] into r1;
    get.or_use storage_demo.aleo/tiny__[false] 0u8 into r2;
    cast true r2 into r3 as Optional__3aph2JMPtnA;
    ternary r1 r3.is_some false into r4;
    ternary r1 r2 0u8 into r5;
    cast r4 r5 into r6 as Optional__3aph2JMPtnA;
    assert.eq r6.is_some true;
    is.eq r6.val 255u8 into r7;
    assert.eq r7 true;

function set_and_read_signed:
    call storage_demo.aleo/set_signed -1234i64 into r0;
    async set_and_read_signed r0 into r1;
    output r1 as test_storage_demo.aleo/set_and_read_signed.future;

finalize set_and_read_signed:
    input r0 as storage_demo.aleo/set_signed.future;
    await r0;
    contains storage_demo.aleo/signed__[false] into r1;
    get.or_use storage_demo.aleo/signed__[false] 0i64 into r2;
    cast true r2 into r3 as Optional__GfbJUku6XoG;
    ternary r1 r3.is_some false into r4;
    ternary r1 r2 0i64 into r5;
    cast r4 r5 into r6 as Optional__GfbJUku6XoG;
    assert.eq r6.is_some true;
    is.eq r6.val -1234i64 into r7;
    assert.eq r7 true;

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
    cast true r3 into r4 as Optional__6wU7KHJhSvv;
    ternary r2 r4.is_some false into r5;
    ternary r2 r3 aleo1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3ljyzc into r6;
    cast r5 r6 into r7 as Optional__6wU7KHJhSvv;
    assert.eq r7.is_some true;
    is.eq r7.val r1 into r8;
    assert.eq r8 true;

function set_and_read_marker:
    call storage_demo.aleo/set_marker 7field into r0;
    async set_and_read_marker r0 into r1;
    output r1 as test_storage_demo.aleo/set_and_read_marker.future;

finalize set_and_read_marker:
    input r0 as storage_demo.aleo/set_marker.future;
    await r0;
    contains storage_demo.aleo/marker__[false] into r1;
    get.or_use storage_demo.aleo/marker__[false] 0field into r2;
    cast true r2 into r3 as Optional__7o6su2Uhzht;
    ternary r1 r3.is_some false into r4;
    ternary r1 r2 0field into r5;
    cast r4 r5 into r6 as Optional__7o6su2Uhzht;
    assert.eq r6.is_some true;
    is.eq r6.val 7field into r7;
    assert.eq r7 true;

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
    cast true r3 into r4 as Optional__9SsWRr2WG5s;
    ternary r2 r4.is_some false into r5;
    ternary r2 r3 0group into r6;
    cast r5 r6 into r7 as Optional__9SsWRr2WG5s;
    assert.eq r7.is_some true;
    is.eq r7.val r1 into r8;
    assert.eq r8 true;

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
    cast true r3 into r4 as Optional__7G49nMxvEkY;
    cast 0i64 0i64 into r5 as storage_demo.aleo/Point;
    ternary r1 r3.x r5.x into r6;
    ternary r1 r3.y r5.y into r7;
    cast r6 r7 into r8 as storage_demo.aleo/Point;
    ternary r1 r4.is_some false into r9;
    cast r9 r8 into r10 as Optional__7G49nMxvEkY;
    assert.eq r10.is_some true;
    is.eq r10.val.x -5i64 into r11;
    assert.eq r11 true;
    is.eq r10.val.y 9i64 into r12;
    assert.eq r12 true;

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
    cast true r3 into r4 as Optional__BnktEVOqVZ2;
    ternary r1 r3[0u32] r2[0u32] into r5;
    ternary r1 r3[1u32] r2[1u32] into r6;
    ternary r1 r3[2u32] r2[2u32] into r7;
    ternary r1 r3[3u32] r2[3u32] into r8;
    cast r5 r6 r7 r8 into r9 as [u32; 4u32];
    ternary r1 r4.is_some false into r10;
    cast r10 r9 into r11 as Optional__BnktEVOqVZ2;
    assert.eq r11.is_some true;
    is.eq r11.val[0u32] 1u32 into r12;
    assert.eq r12 true;
    is.eq r11.val[1u32] 2u32 into r13;
    assert.eq r13 true;
    is.eq r11.val[2u32] 3u32 into r14;
    assert.eq r14 true;
    is.eq r11.val[3u32] 4u32 into r15;
    assert.eq r15 true;

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
    cast true r4 into r5 as Optional__JzunLORyB8U;
    ternary r3 r5.is_some false into r6;
    ternary r3 r4 0u32 into r7;
    cast r6 r7 into r8 as Optional__JzunLORyB8U;
    ternary r8.is_some r8.val 0u32 into r9;
    is.eq r9 0u32 into r10;
    assert.eq r10 true;

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
    cast true r7 into r8 as Optional__JzunLORyB8U;
    ternary r6 r8.is_some false into r9;
    ternary r6 r7 0u32 into r10;
    cast r9 r10 into r11 as Optional__JzunLORyB8U;
    assert.eq r11.is_some true;
    is.eq r11.val 10u32 into r12;
    assert.eq r12 true;
    get.or_use storage_demo.aleo/history__len__[false] 0u32 into r13;
    lt 1u32 r13 into r14;
    get.or_use storage_demo.aleo/history__[1u32] 0u32 into r15;
    cast true r15 into r16 as Optional__JzunLORyB8U;
    ternary r14 r16.is_some false into r17;
    ternary r14 r15 0u32 into r18;
    cast r17 r18 into r19 as Optional__JzunLORyB8U;
    assert.eq r19.is_some true;
    is.eq r19.val 20u32 into r20;
    assert.eq r20 true;
    get.or_use storage_demo.aleo/history__len__[false] 0u32 into r21;
    lt 2u32 r21 into r22;
    get.or_use storage_demo.aleo/history__[2u32] 0u32 into r23;
    cast true r23 into r24 as Optional__JzunLORyB8U;
    ternary r22 r24.is_some false into r25;
    ternary r22 r23 0u32 into r26;
    cast r25 r26 into r27 as Optional__JzunLORyB8U;
    assert.eq r27.is_some true;
    is.eq r27.val 30u32 into r28;
    assert.eq r28 true;
    get.or_use storage_demo.aleo/history__len__[false] 0u32 into r29;
    lt 3u32 r29 into r30;
    get.or_use storage_demo.aleo/history__[3u32] 0u32 into r31;
    cast true r31 into r32 as Optional__JzunLORyB8U;
    ternary r30 r32.is_some false into r33;
    ternary r30 r31 0u32 into r34;
    cast r33 r34 into r35 as Optional__JzunLORyB8U;
    ternary r35.is_some r35.val 0u32 into r36;
    is.eq r36 0u32 into r37;
    assert.eq r37 true;

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
    cast true r7 into r8 as Optional__JzunLORyB8U;
    ternary r6 r8.is_some false into r9;
    ternary r6 r7 0u32 into r10;
    cast r9 r10 into r11 as Optional__JzunLORyB8U;
    assert.eq r11.is_some true;
    is.eq r11.val 7u32 into r12;
    assert.eq r12 true;

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
    cast true r7 into r8 as Optional__JzunLORyB8U;
    ternary r6 r8.is_some false into r9;
    ternary r6 r7 0u32 into r10;
    cast r9 r10 into r11 as Optional__JzunLORyB8U;
    assert.eq r11.is_some true;
    is.eq r11.val 100u32 into r12;
    assert.eq r12 true;
    get.or_use storage_demo.aleo/history__len__[false] 0u32 into r13;
    lt 1u32 r13 into r14;
    get.or_use storage_demo.aleo/history__[1u32] 0u32 into r15;
    cast true r15 into r16 as Optional__JzunLORyB8U;
    ternary r14 r16.is_some false into r17;
    ternary r14 r15 0u32 into r18;
    cast r17 r18 into r19 as Optional__JzunLORyB8U;
    assert.eq r19.is_some true;
    is.eq r19.val 999u32 into r20;
    assert.eq r20 true;

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
    cast true r9 into r10 as Optional__JzunLORyB8U;
    ternary r8 r10.is_some false into r11;
    ternary r8 r9 0u32 into r12;
    cast r11 r12 into r13 as Optional__JzunLORyB8U;
    assert.eq r13.is_some true;
    is.eq r13.val 1u32 into r14;
    assert.eq r14 true;
    get.or_use storage_demo.aleo/history__len__[false] 0u32 into r15;
    lt 1u32 r15 into r16;
    get.or_use storage_demo.aleo/history__[1u32] 0u32 into r17;
    cast true r17 into r18 as Optional__JzunLORyB8U;
    ternary r16 r18.is_some false into r19;
    ternary r16 r17 0u32 into r20;
    cast r19 r20 into r21 as Optional__JzunLORyB8U;
    assert.eq r21.is_some true;
    is.eq r21.val 4u32 into r22;
    assert.eq r22 true;
    get.or_use storage_demo.aleo/history__len__[false] 0u32 into r23;
    lt 2u32 r23 into r24;
    get.or_use storage_demo.aleo/history__[2u32] 0u32 into r25;
    cast true r25 into r26 as Optional__JzunLORyB8U;
    ternary r24 r26.is_some false into r27;
    ternary r24 r25 0u32 into r28;
    cast r27 r28 into r29 as Optional__JzunLORyB8U;
    assert.eq r29.is_some true;
    is.eq r29.val 3u32 into r30;
    assert.eq r30 true;

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
    cast true r7 into r8 as Optional__JzunLORyB8U;
    ternary r6 r8.is_some false into r9;
    ternary r6 r7 0u32 into r10;
    cast r9 r10 into r11 as Optional__JzunLORyB8U;
    ternary r11.is_some r11.val 0u32 into r12;
    is.eq r12 0u32 into r13;
    assert.eq r13 true;

function unwrap_unset_singleton_halts:
    async unwrap_unset_singleton_halts into r0;
    output r0 as test_storage_demo.aleo/unwrap_unset_singleton_halts.future;

finalize unwrap_unset_singleton_halts:
    contains storage_demo.aleo/counter__[false] into r0;
    get.or_use storage_demo.aleo/counter__[false] 0u64 into r1;
    cast true r1 into r2 as Optional__DmN5CQ9hzeK;
    ternary r0 r2.is_some false into r3;
    ternary r0 r1 0u64 into r4;
    cast r3 r4 into r5 as Optional__DmN5CQ9hzeK;
    assert.eq r5.is_some true;
    is.eq r5.val 0u64 into r6;
    assert.eq r6 true;

function unwrap_oob_get_halts:
    async unwrap_oob_get_halts into r0;
    output r0 as test_storage_demo.aleo/unwrap_oob_get_halts.future;

finalize unwrap_oob_get_halts:
    get.or_use storage_demo.aleo/history__len__[false] 0u32 into r0;
    lt 0u32 r0 into r1;
    get.or_use storage_demo.aleo/history__[0u32] 0u32 into r2;
    cast true r2 into r3 as Optional__JzunLORyB8U;
    ternary r1 r3.is_some false into r4;
    ternary r1 r2 0u32 into r5;
    cast r4 r5 into r6 as Optional__JzunLORyB8U;
    assert.eq r6.is_some true;
    is.eq r6.val 0u32 into r7;
    assert.eq r7 true;

constructor:
    assert.eq edition 0u16;
