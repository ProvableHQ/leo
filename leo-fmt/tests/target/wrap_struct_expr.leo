program test.aleo {
    struct Point {
        x: u64,
        y: u64,
    }

    struct TransferInfo {
        sender_amount: u64,
        receiver_amount: u64,
        transfer_fee: u64,
    }

    fn main(public a: u64, public b: u64) -> u64 {
        let p: Point = Point { x: 1u64, y: 2u64 };
        let info: TransferInfo = TransferInfo {
            sender_amount: a,
            receiver_amount: b,
            transfer_fee: 10u64,
        };
        return p.x + info.sender_amount;
    }
}
