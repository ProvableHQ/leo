program b28923.aleo {
    @noupgrade
    constructor() {}

    fn main(message_raw: [u8; 4]) -> [u8; 4] {
        const length: u32 = 4;
        let message = [0u8; length];
        for i: u32 in 0u32..length{
            message[i] = message_raw[i];
        }
        return message;
    }
}
