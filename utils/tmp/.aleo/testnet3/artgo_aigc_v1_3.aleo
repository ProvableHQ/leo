import credits.aleo;

program artgo_aigc_v1_3.aleo;

struct Field2:
    first as field;
    second as field;

struct NftHash:
    minter as address;
    prompt as Field2;
    nonce as field;

struct NftInfo:
    holder as address;
    minter as address;
    hash as field;
    uri as Field2;

struct Order:
    holder as address;
    nft_id as field;
    order_type as u8;
    amount as u64;
    status as u8;
    deadline as field;

struct Bid:
    bidder as address;
    nft_id as field;
    amount as u64;

record BidRecord:
    owner as address.private;
    bid_type as u8.private;
    bidder as address.private;
    nft_id as field.private;
    amount as u64.private;

record Nft:
    owner as address.private;
    minter as address.private;
    prompt as Field2.private;
    nonce as field.private;
    uri as Field2.private;

mapping nfts:
    key as field.public;
    value as NftInfo.public;

mapping tokenId:
    key as boolean.public;
    value as field.public;

mapping admin:
    key as boolean.public;
    value as address.public;

mapping fees:
    key as boolean.public;
    value as u64.public;

mapping orders:
    key as field.public;
    value as Order.public;

mapping bids:
    key as field.public;
    value as Bid.public;

function mint_nft:
    input r0 as Field2.private;
    input r1 as Field2.public;
    input r2 as field.public;
    cast self.caller r0 r2 into r3 as NftHash;
    hash.bhp256 r3 into r4 as field;
    cast self.caller self.caller r0 r2 r1 into r5 as Nft.record;
    async mint_nft self.caller r4 r1 into r6;
    output r5 as Nft.record;
    output r6 as artgo_aigc_v1_3.aleo/mint_nft.future;

finalize mint_nft:
    input r0 as address.public;
    input r1 as field.public;
    input r2 as Field2.public;
    get.or_use tokenId[true] 1field into r3;
    add r3 1field into r4;
    set r4 into tokenId[true];
    cast r0 r0 r1 r2 into r5 as NftInfo;
    set r5 into nfts[r3];

function transfer_private_nft:
    input r0 as field.private;
    input r1 as Nft.record;
    input r2 as address.private;
    is.eq r1.owner self.caller into r3;
    assert.eq r3 true ;
    cast r1.minter r1.prompt r1.nonce into r4 as NftHash;
    hash.bhp256 r4 into r5 as field;
    cast r2 r1.minter r1.prompt r1.nonce r1.uri into r6 as Nft.record;
    async transfer_private_nft r0 r5 r1.uri r2 into r7;
    output r6 as Nft.record;
    output r7 as artgo_aigc_v1_3.aleo/transfer_private_nft.future;

finalize transfer_private_nft:
    input r0 as field.public;
    input r1 as field.public;
    input r2 as Field2.public;
    input r3 as address.public;
    get nfts[r0] into r4;
    assert.eq r4.hash r1 ;
    assert.eq r4.uri r2 ;
    cast r3 r4.minter r4.hash r4.uri into r5 as NftInfo;
    set r5 into nfts[r0];

function place_order:
    input r0 as field.public;
    input r1 as Nft.record;
    input r2 as u8.public;
    input r3 as u64.public;
    input r4 as field.public;
    input r5 as address.public;
    is.eq r2 1u8 into r6;
    assert.eq r6 true ;
    gt r3 0u64 into r7;
    lte r3 18446744073709551615u64 into r8;
    and r7 r8 into r9;
    assert.eq r9 true ;
    cast r1.minter r1.prompt r1.nonce into r10 as NftHash;
    hash.bhp256 r10 into r11 as field;
    cast self.caller r0 r2 r3 1u8 0field into r12 as Order;
    cast r5 r1.minter r1.prompt r1.nonce r1.uri into r13 as Nft.record;
    async place_order r5 r0 r11 r1.uri r12 into r14;
    output r13 as Nft.record;
    output r14 as artgo_aigc_v1_3.aleo/place_order.future;

finalize place_order:
    input r0 as address.public;
    input r1 as field.public;
    input r2 as field.public;
    input r3 as Field2.public;
    input r4 as Order.public;
    contains admin[true] into r5;
    assert.eq r5 true ;
    get admin[true] into r6;
    assert.eq r6 r0 ;
    get nfts[r1] into r7;
    assert.eq r7.hash r2 ;
    assert.eq r7.uri r3 ;
    contains fees[true] into r8;
    assert.eq r8 true ;
    get fees[true] into r9;
    gte r4.amount r9 into r10;
    assert.eq r10 true ;
    contains orders[r1] into r11;
    not r11 into r12;
    assert.eq r12 true ;
    set r4 into orders[r1];

function update_order:
    input r0 as field.public;
    input r1 as u64.public;
    gt r1 0u64 into r2;
    lte r1 18446744073709551615u64 into r3;
    and r2 r3 into r4;
    assert.eq r4 true ;
    async update_order self.caller r0 r1 into r5;
    output r5 as artgo_aigc_v1_3.aleo/update_order.future;

finalize update_order:
    input r0 as address.public;
    input r1 as field.public;
    input r2 as u64.public;
    get orders[r1] into r3;
    is.eq r3.status 1u8 into r4;
    assert.eq r4 true ;
    is.eq r0 r3.holder into r5;
    assert.eq r5 true ;
    contains fees[true] into r6;
    assert.eq r6 true ;
    get fees[true] into r7;
    gte r2 r7 into r8;
    assert.eq r8 true ;
    cast r3.holder r3.nft_id r3.order_type r2 r3.status r3.deadline into r9 as Order;
    set r9 into orders[r1];

function cancel_order:
    input r0 as u64.public;
    input r1 as field.public;
    input r2 as address.public;
    cast r2 2u8 self.caller r1 0u64 into r3 as BidRecord.record;
    call credits.aleo/transfer_public r2 r0 into r4;
    async cancel_order r4 self.caller r1 r0 r2 into r5;
    output r3 as BidRecord.record;
    output r5 as artgo_aigc_v1_3.aleo/cancel_order.future;

finalize cancel_order:
    input r0 as credits.aleo/transfer_public.future;
    input r1 as address.public;
    input r2 as field.public;
    input r3 as u64.public;
    input r4 as address.public;
    await r0;
    contains fees[true] into r5;
    assert.eq r5 true ;
    get fees[true] into r6;
    assert.eq r6 r3 ;
    get orders[r2] into r7;
    is.eq r7.status 1u8 into r8;
    assert.eq r8 true ;
    is.eq r1 r7.holder into r9;
    assert.eq r9 true ;
    contains admin[true] into r10;
    assert.eq r10 true ;
    get admin[true] into r11;
    assert.eq r11 r4 ;
    cast r7.holder r7.nft_id r7.order_type r7.amount 2u8 r7.deadline into r12 as Order;
    set r12 into orders[r2];

function handle_cancel_order:
    input r0 as field.public;
    input r1 as Nft.record;
    input r2 as address.public;
    cast r2 r1.minter r1.prompt r1.nonce r1.uri into r3 as Nft.record;
    cast r1.minter r1.prompt r1.nonce into r4 as NftHash;
    hash.bhp256 r4 into r5 as field;
    async handle_cancel_order r0 r5 r1.uri r2 self.caller into r6;
    output r3 as Nft.record;
    output r6 as artgo_aigc_v1_3.aleo/handle_cancel_order.future;

finalize handle_cancel_order:
    input r0 as field.public;
    input r1 as field.public;
    input r2 as Field2.public;
    input r3 as address.public;
    input r4 as address.public;
    contains admin[true] into r5;
    assert.eq r5 true ;
    get admin[true] into r6;
    assert.eq r6 r4 ;
    get nfts[r0] into r7;
    assert.eq r7.hash r1 ;
    assert.eq r7.uri r2 ;
    get orders[r0] into r8;
    is.eq r8.status 2u8 into r9;
    assert.eq r9 true ;
    is.eq r3 r8.holder into r10;
    assert.eq r10 true ;
    remove bids[r0];
    remove orders[r0];

function bid_order:
    input r0 as field.public;
    input r1 as u64.public;
    input r2 as address.public;
    gt r1 0u64 into r3;
    lte r1 18446744073709551615u64 into r4;
    and r3 r4 into r5;
    assert.eq r5 true ;
    cast r2 1u8 self.caller r0 r1 into r6 as BidRecord.record;
    call credits.aleo/transfer_public r2 r1 into r7;
    async bid_order r7 self.caller r0 r1 r2 into r8;
    output r6 as BidRecord.record;
    output r8 as artgo_aigc_v1_3.aleo/bid_order.future;

finalize bid_order:
    input r0 as credits.aleo/transfer_public.future;
    input r1 as address.public;
    input r2 as field.public;
    input r3 as u64.public;
    input r4 as address.public;
    await r0;
    contains admin[true] into r5;
    assert.eq r5 true ;
    get admin[true] into r6;
    assert.eq r6 r4 ;
    cast r1 r2 r3 into r7 as Bid;
    set r7 into bids[r2];
    contains orders[r2] into r8;
    assert.eq r8 true ;
    get orders[r2] into r9;
    is.eq r9.status 1u8 into r10;
    assert.eq r10 true ;
    gte r3 r9.amount into r11;
    assert.eq r11 true ;
    cast r9.holder r9.nft_id r9.order_type r9.amount 3u8 0field into r12 as Order;
    set r12 into orders[r2];

function finish_order:
    input r0 as field.public;
    input r1 as Nft.record;
    input r2 as address.public;
    input r3 as u64.public;
    input r4 as address.public;
    input r5 as u64.public;
    input r6 as address.public;
    gte r5 r3 into r7;
    lte r5 18446744073709551615u64 into r8;
    and r7 r8 into r9;
    assert.eq r9 true ;
    assert.eq self.caller r6 ;
    sub r5 r3 into r10;
    call credits.aleo/transfer_public r2 r10 into r11;
    cast r1.minter r1.prompt r1.nonce into r12 as NftHash;
    hash.bhp256 r12 into r13 as field;
    cast r4 r1.minter r1.prompt r1.nonce r1.uri into r14 as Nft.record;
    async finish_order r11 r0 r2 r13 r1.uri r4 r5 r3 r6 into r15;
    output r14 as Nft.record;
    output r15 as artgo_aigc_v1_3.aleo/finish_order.future;

finalize finish_order:
    input r0 as credits.aleo/transfer_public.future;
    input r1 as field.public;
    input r2 as address.public;
    input r3 as field.public;
    input r4 as Field2.public;
    input r5 as address.public;
    input r6 as u64.public;
    input r7 as u64.public;
    input r8 as address.public;
    await r0;
    contains admin[true] into r9;
    assert.eq r9 true ;
    get admin[true] into r10;
    assert.eq r10 r8 ;
    contains fees[true] into r11;
    assert.eq r11 true ;
    get fees[true] into r12;
    assert.eq r12 r7 ;
    contains orders[r1] into r13;
    assert.eq r13 true ;
    get orders[r1] into r14;
    assert.eq r14.status 3u8 ;
    get nfts[r1] into r15;
    assert.eq r15.hash r3 ;
    assert.eq r15.uri r4 ;
    assert.eq r15.holder r2 ;
    contains bids[r1] into r16;
    assert.eq r16 true ;
    get bids[r1] into r17;
    assert.eq r17.bidder r5 ;
    is.eq r17.amount r6 into r18;
    assert.eq r18 true ;
    cast r5 r15.minter r15.hash r15.uri into r19 as NftInfo;
    set r19 into nfts[r1];
    remove bids[r1];
    remove orders[r1];

function set_admin:
    input r0 as address.private;
    async set_admin self.caller r0 into r1;
    output r1 as artgo_aigc_v1_3.aleo/set_admin.future;

finalize set_admin:
    input r0 as address.public;
    input r1 as address.public;
    get.or_use admin[true] aleo12n3kz2wzrljvhp9zkn9zety54wz8mlmlakzzkrs6znaz028wjczqpe8x3j into r2;
    assert.eq r2 r0 ;
    set r1 into admin[true];

function set_fee:
    input r0 as u64.private;
    async set_fee self.caller r0 into r1;
    output r1 as artgo_aigc_v1_3.aleo/set_fee.future;

finalize set_fee:
    input r0 as address.public;
    input r1 as u64.public;
    get admin[true] into r2;
    assert.eq r0 r2 ;
    set r1 into fees[true];
